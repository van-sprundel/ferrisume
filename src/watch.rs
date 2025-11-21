use log::{debug, error, info, warn};
use notify::{RecursiveMode, Watcher};
use serde_json;
use std::fs::{self, File};
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::path::Path;
use tiny_http::{Header, Response, Server};

use crate::core::{generate_html, ThemeManager};

type SseClients = Arc<Mutex<Vec<Sender<String>>>>;

/// A reader that reads from a channel to implement SSE streaming
struct SseReader {
    rx: Receiver<String>,
    buffer: Vec<u8>,
    pos: usize,
}

impl SseReader {
    fn new(rx: Receiver<String>) -> Self {
        Self {
            rx,
            buffer: b": connected\n\n".to_vec(),
            pos: 0,
        }
    }
}

impl Read for SseReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // If we have buffered data, return it first
        if self.pos < self.buffer.len() {
            let remaining = self.buffer.len() - self.pos;
            let to_copy = remaining.min(buf.len());
            buf[..to_copy].copy_from_slice(&self.buffer[self.pos..self.pos + to_copy]);
            self.pos += to_copy;

            // If we've consumed all buffered data, clear the buffer
            if self.pos >= self.buffer.len() {
                self.buffer.clear();
                self.pos = 0;
            }

            return Ok(to_copy);
        }

        // Wait for next message or keepalive timeout
        match self.rx.recv_timeout(Duration::from_secs(30)) {
            Ok(msg) => {
                // Format as SSE message
                self.buffer = format!("data: {}\n\n", msg).into_bytes();
                self.pos = 0;
                // Recursively call read to return the data
                self.read(buf)
            }
            Err(_) => {
                // Send keepalive
                self.buffer = b": keepalive\n\n".to_vec();
                self.pos = 0;
                self.read(buf)
            }
        }
    }
}

pub fn watch_command(
    theme_name: &str,
    http_port: u16,
    _ws_port: u16, // Kept for API compatibility, but SSE uses same port as HTTP
) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = channel();
    let mut watcher = notify::PollWatcher::new(
        tx,
        notify::Config::default()
            .with_poll_interval(Duration::from_millis(100))
            .with_compare_contents(true),
    )?;

    let mut theme_manager = ThemeManager::new();

    theme_manager.set_theme(theme_name)?;
    let theme = theme_manager
        .get_current_theme()
        .ok_or_else(|| format!("Theme '{}' not found", theme_name))?;

    if theme_name.contains('/') || theme_name.contains('\\') {
        println!("Using theme from path: {}", theme.path.display());
    } else {
        println!("Using theme: {}", theme_name);
    }

    let json_file_path = Path::new("resume.json");
    if !json_file_path.exists() {
        let resume = crate::domain::Resume::default();
        let resume_json = serde_json::to_string_pretty(&resume)?;
        fs::write(json_file_path, resume_json)?;
    }
    watcher.watch(json_file_path, RecursiveMode::NonRecursive)?;

    let theme_path = theme.path.clone();
    if theme_path.exists() {
        println!("Watching theme directory: {}", theme.path.display());
        println!("Any changes to theme files will be automatically detected");
        watcher.watch(&theme_path, RecursiveMode::Recursive)?;
    }

    let themes_path = Path::new("themes");
    if themes_path.exists() {
        watcher.watch(themes_path, RecursiveMode::Recursive)?;
    }

    let html_dir_path = tempfile::Builder::new()
        .prefix("ferrisume-watch")
        .tempdir()?;
    let html_file_path = html_dir_path.path().join("resume.htm");
    let html_file_path_clone = html_file_path.clone();

    match rebuild_resume(&theme_manager, json_file_path, &html_file_path) {
        Ok(_) => info!("Initial resume generated successfully"),
        Err(e) => {
            warn!("Error generating initial resume: {}", e);
            fs::write(
                &html_file_path,
                format!("<h1>Error building resume</h1><p>{}</p>", e),
            )?;
        }
    }

    let http_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), http_port);
    let http_server = match try_bind_server(&http_addr) {
        Ok(server) => server,
        Err(e) => {
            return Err(format!("HTTP port {} is already in use. Please specify a different port with --http-port or stop the process using this port.
                Error: {}", http_port, e).into());
        }
    };

    println!();
    println!(
        "Live preview started! Open http://127.0.0.1:{} in your browser",
        http_port
    );
    println!("Press Ctrl+C to stop the server");

    // Track SSE clients
    let sse_clients: SseClients = Arc::new(Mutex::new(Vec::new()));
    let sse_clients_http = sse_clients.clone();

    let theme_path_clone = theme_path.clone();
    thread::spawn(move || {
        let html_file_path = html_file_path_clone;
        let theme_path = theme_path_clone;
        for request in http_server.incoming_requests() {
            let url = request.url().to_string();

            // Handle SSE endpoint
            if url == "/events" {
                info!("SSE client connected");
                let (tx, rx): (Sender<String>, Receiver<String>) = channel();

                // Add this client to the list
                {
                    let mut clients = sse_clients_http.lock().unwrap();
                    clients.push(tx);
                }

                // Create SSE response with streaming reader
                let reader = SseReader::new(rx);
                let headers = vec![
                    Header::from_bytes(&b"Content-Type"[..], &b"text/event-stream"[..]).unwrap(),
                    Header::from_bytes(&b"Cache-Control"[..], &b"no-cache"[..]).unwrap(),
                    Header::from_bytes(&b"Connection"[..], &b"keep-alive"[..]).unwrap(),
                    Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap(),
                ];
                let response = Response::new(
                    tiny_http::StatusCode(200),
                    headers,
                    reader,
                    None, // Unknown length for streaming
                    None  // No additional headers
                );

                let _ = request.respond(response);
                info!("SSE client disconnected");
                continue;
            }

            if url.starts_with("/fonts/") || url.starts_with("/css/") || url.starts_with("/assets/")
            {
                let rel_path = url.trim_start_matches('/');
                let theme_file_path = theme_path.join("templates").join(rel_path);

                match File::open(&theme_file_path) {
                    Ok(mut file) => {
                        let mut buffer = Vec::new();
                        if file.read_to_end(&mut buffer).is_ok() {
                            let content_type =
                                match theme_file_path.extension().and_then(|ext| ext.to_str()) {
                                    Some("css") => "text/css",
                                    Some("js") => "application/javascript",
                                    Some("svg") => "image/svg+xml",
                                    Some("png") => "image/png",
                                    Some("jpg") | Some("jpeg") => "image/jpeg",
                                    Some("otf") | Some("ttf") => "font/otf",
                                    Some("woff") => "font/woff",
                                    Some("woff2") => "font/woff2",
                                    _ => "application/octet-stream",
                                };

                            let response =
                                Response::from_data(buffer).with_header(tiny_http::Header {
                                    field: "Content-Type".parse().unwrap(),
                                    value: content_type.parse().unwrap(),
                                });
                            let _ = request.respond(response);
                        } else {
                            let response =
                                Response::from_string("Error reading file").with_status_code(500);
                            let _ = request.respond(response);
                        }
                    }
                    Err(_) => {
                        let response =
                            Response::from_string("File not found").with_status_code(404);
                        let _ = request.respond(response);
                    }
                }
            } else {
                // Serve the resume HTML
                let response = Response::from_data(generate_resume_html(&html_file_path).as_bytes());
                let _ = request.respond(response);
            }
        }
    });

    loop {
        match rx.recv() {
            Ok(_) => {
                info!("Change detected, rebuilding...");
                match rebuild_resume(&theme_manager, json_file_path, &html_file_path) {
                    Ok(_) => {
                        broadcast_reload(&sse_clients);
                    }
                    Err(e) => {
                        warn!("Error building resume: {}", e);
                        fs::write(
                            &html_file_path,
                            format!("<h1>Error building resume</h1><p>{}</p>", e),
                        )
                        .unwrap_or_else(|write_err| {
                            error!("Failed to write error to file: {}", write_err)
                        });
                        broadcast_reload(&sse_clients);
                    }
                }
            }
            Err(e) => error!("Watch error: {:?}", e),
        }
    }
}

fn broadcast_reload(clients: &SseClients) {
    let mut clients_lock = clients.lock().unwrap();
    // Remove disconnected clients and send to active ones
    clients_lock.retain(|client| client.send("reload".to_string()).is_ok());
    debug!("Broadcasted reload to {} clients", clients_lock.len());
}

fn try_bind_server(addr: &SocketAddr) -> Result<Server, Box<dyn std::error::Error>> {
    match Server::http(addr.to_string()) {
        Ok(server) => Ok(server),
        Err(e) => Err(format!("Failed to bind server to {}: {}", addr, e).into()),
    }
}

fn rebuild_resume(
    theme_manager: &ThemeManager,
    resume_json_file: &Path,
    resume_html_file: &Path,
) -> Result<(), String> {
    let resume_json = std::fs::read_to_string(resume_json_file)
        .map_err(|e| format!("Error reading resume file: {}", e))?;
    let resume_json =
        &serde_json::from_str(&resume_json).map_err(|e| format!("Error parsing JSON: {}", e))?;

    match generate_html(theme_manager, resume_json) {
        Ok(content) => {
            debug!("Generated html: \n{}", &content);
            fs::write(resume_html_file, content)
        }
        Err(e) => fs::write("resume.htm", format!("<pre>{}</pre>", e)),
    }
    .map_err(|e| format!("error writing html file: {}", e))?;

    Ok(())
}

fn generate_resume_html(html_file_path: &Path) -> String {
    let content = fs::read_to_string(html_file_path)
        .unwrap_or_else(|_| "<p>Resume not generated yet.</p>".to_string());

    format!(
        r#"
    {}
    <script type="text/javascript">
        const eventSource = new EventSource('/events');

        eventSource.onmessage = function(event) {{
            if (event.data === 'reload') {{
                console.log('Reloading due to changes...');
                location.reload();
            }}
        }};

        eventSource.onerror = function(error) {{
            console.error('SSE connection error, will retry automatically');
        }};

        console.log('Ferrisume live preview active - watching for changes');
    </script>
"#,
        content
    )
}
