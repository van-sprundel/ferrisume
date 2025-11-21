use log::{debug, error, info, warn};
use notify::{RecursiveMode, Watcher};
use serde_json;
use std::fs::{self, File};
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{path::Path, sync::mpsc::channel};
use tiny_http::{Response, Server};
use ws::{listen, CloseCode, Handler, Handshake, Message, Result as WsResult};

use crate::core::{generate_html, ThemeManager};

struct WSServer;

impl Handler for WSServer {
    fn on_open(&mut self, _: Handshake) -> WsResult<()> {
        info!("Client connected");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> WsResult<()> {
        info!("Received message: {}", msg);
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        info!("Client disconnected: {:?} {}", code, reason);
    }
}

pub fn watch_command(
    theme_name: &str,
    http_port: u16,
    ws_port: u16,
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
        println!("Watching theme directory: {}", theme_path.display());
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
    let html_file_path_clone = html_file_path.clone(); // make a copy for the request thread to use

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

    let ws_addr = format!("127.0.0.1:{}", ws_port);
    match TcpListener::bind(&ws_addr) {
        Ok(_) => { /* port is available */ }
        Err(e) => {
            return Err(format!("WebSocket port {} is already in use. Please specify a different port with --ws-port or stop the process using this port.
                Error: {}", ws_port, e).into());
        }
    };

    println!();
    println!(
        "Live preview started! Open http://127.0.0.1:{} in your browser",
        http_port
    );
    println!("Press Ctrl+C to stop the server");

    let websocket_server = Arc::new(Mutex::new(None));
    let websocket_server_clone = websocket_server.clone();
    let ws_port_copy = ws_port;
    thread::spawn(move || loop {
        info!("Starting WebSocket server on port {}", ws_port_copy);
        if let Err(e) = listen(&ws_addr, |out| {
            let mut server = websocket_server_clone.lock().unwrap();
            *server = Some(out.clone());
            WSServer
        }) {
            error!("WebSocket server error: {:?}", e);
            thread::sleep(Duration::from_secs(5));
        }
    });

    let theme_path_clone = theme_path.clone();
    let ws_port_for_html = ws_port;
    thread::spawn(move || {
        let html_file_path = html_file_path_clone;
        let theme_path = theme_path_clone;
        let ws_port = ws_port_for_html;
        for request in http_server.incoming_requests() {
            let url = request.url().to_string();

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
                let response =
                    Response::from_data(generate_resume_html(&html_file_path, ws_port).as_bytes());
                let _ = request.respond(response);
            }
        }
    });

    loop {
        match rx.recv() {
            Ok(_) => {
                info!("Change detected, rebuilding...");
                match rebuild_resume(&theme_manager, json_file_path, &html_file_path) {
                    Ok(_) => reload_socket(&websocket_server),
                    Err(e) => {
                        warn!("Error building resume: {}", e);
                        fs::write(
                            &html_file_path,
                            format!("<h1>Error building resume</h1><p>{}</p>", e),
                        )
                        .unwrap_or_else(|write_err| {
                            error!("Failed to write error to file: {}", write_err)
                        });
                        reload_socket(&websocket_server);
                    }
                }
            }
            Err(e) => error!("Watch error: {:?}", e),
        }
    }
}

fn try_bind_server(addr: &SocketAddr) -> Result<Server, Box<dyn std::error::Error>> {
    match Server::http(addr.to_string()) {
        Ok(server) => Ok(server),
        Err(e) => Err(format!("Failed to bind server to {}: {}", addr, e).into()),
    }
}

fn reload_socket(websocket_server: &Arc<Mutex<Option<ws::Sender>>>) {
    let server = websocket_server.lock().unwrap();
    if let Some(ref out) = *server {
        if let Err(e) = out.send("reload") {
            error!("Error sending reload message: {:?}", e);
        }
    } else {
        warn!("WebSocket server not available");
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

fn generate_resume_html(html_file_path: &Path, ws_port: u16) -> String {
    let content = fs::read_to_string(html_file_path)
        .unwrap_or_else(|_| "<p>Resume not generated yet.</p>".to_string());

    format!(
        r#"
    {}
    <script type="text/javascript">
        var socket = new WebSocket("ws://localhost:{}");

        socket.onmessage = function(event) {{
            if (event.data === "reload") {{
                console.log("Reloading due to changes...");
                location.reload();
            }}
        }};

        console.log("Ferrisume live preview active - watching for changes");
    </script>
"#,
        content, ws_port
    )
}
