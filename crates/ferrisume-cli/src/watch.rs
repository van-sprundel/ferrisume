use ferrisume_core::{generate_html, ThemeManager};
use log::{error, info, warn};
use notify::{RecursiveMode, Watcher};
use std::fs::{self, File};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{path::Path, sync::mpsc::channel};
use tiny_http::Response;
use ws::{listen, CloseCode, Handler, Handshake, Message, Result as WsResult, Sender};

struct WSServer {
    out: Sender,
}

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

pub fn watch_command() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = channel();
    let mut watcher = notify::PollWatcher::new(
        tx,
        notify::Config::default()
            .with_poll_interval(Duration::from_millis(100))
            .with_compare_contents(true),
    )?;

    let theme_manager = ThemeManager::new();

    let resume_path = Path::new("resume.json");
    if !resume_path.exists() {
        File::create(resume_path)?;
    }
    watcher.watch(resume_path, RecursiveMode::NonRecursive)?;

    let themes_path = Path::new("themes");
    if themes_path.exists() {
        watcher.watch(themes_path, RecursiveMode::Recursive)?;
    }

    let server = tiny_http::Server::http("127.0.0.1:8000").expect("Couldn't start http server");
    info!("Serving resume at http://127.0.0.1:8000");

    let websocket_server = Arc::new(Mutex::new(None));
    let websocket_server_clone = websocket_server.clone();
    thread::spawn(move || loop {
        info!("Starting WebSocket server");
        if let Err(e) = listen("127.0.0.1:9000", |out| {
            let mut server = websocket_server_clone.lock().unwrap();
            *server = Some(out.clone());
            WSServer { out }
        }) {
            error!("WebSocket server error: {:?}", e);
            thread::sleep(Duration::from_secs(5));
        }
    });

    thread::spawn(move || {
        for request in server.incoming_requests() {
            let response = Response::from_data(generate_resume_html().as_bytes());
            let _ = request.respond(response);
        }
    });

    loop {
        match rx.recv() {
            Ok(_) => {
                info!("Change detected, rebuilding...");
                match rebuild_resume(&theme_manager, resume_path) {
                    Ok(_) => reload_socket(&websocket_server),
                    Err(e) => warn!("Error building resume: {e}"),
                }
            }
            Err(e) => error!("Watch error: {:?}", e),
        }
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
    resume_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let resume_json = std::fs::read_to_string(resume_path)?;
    let resume_json = &serde_json::from_str(&resume_json)?;

    let content = generate_html(theme_manager, resume_json)?;
    fs::write("resume.htm", content)?;

    Ok(())
}

fn generate_resume_html() -> String {
    let content = fs::read_to_string("resume.htm")
        .unwrap_or_else(|_| "Resume not generated yet.".to_string());

    format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Resume viewer</title>
</head>
<body>
    <main>
    {}
    </main>
    <script type="text/javascript">
        var socket = new WebSocket("ws://localhost:9000");

        socket.onmessage = function(event) {{
            if (event.data === "reload") {{
                location.reload();
            }}
        }};
    </script>
</body>
</html>
"#,
        content
    )
}
