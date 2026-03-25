//! `prism serve` — Start a local web server to host the Prism Web UI.

use clap::Args;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Arguments for the serve command.
#[derive(Args)]
pub struct ServeArgs {
    /// Port to bind the server to (default: 3000).
    #[arg(long, short = 'p', default_value = "3000")]
    pub port: u16,

    /// Host to bind the server to (default: 127.0.0.1).
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// Open the browser automatically.
    #[arg(long)]
    pub open: bool,
}

/// Execute the serve command.
pub async fn run(args: ServeArgs) -> anyhow::Result<()> {
    let host = args.host;
    let port = args.port;
    let addr = format!("{}:{}", host, port);

    println!("🌐 Starting Prism Web Server...");
    println!("📍 Server will be available at: http://{}:{}", host, port);
    
    if args.open {
        if let Err(e) = open_browser(&format!("http://{}:{}", host, port)) {
            eprintln!("⚠️  Could not open browser: {}", e);
        }
    }

    println!("🚀 Press Ctrl+C to stop the server");

    // Create the TCP listener
    let listener = TcpListener::bind(&addr)
        .map_err(|e| anyhow::anyhow!("Failed to bind to {}: {}", addr, e))?;

    println!("✅ Server listening on {}", addr);

    // Handle incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}

/// Handle an incoming HTTP connection.
fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    let mut buffer = [0; 1024];
    
    // Read the request
    stream.read(&mut buffer)?;
    
    // Parse the HTTP request
    let request = String::from_utf8_lossy(&buffer[..]);
    let lines: Vec<&str> = request.lines().collect();
    
    if lines.is_empty() {
        return Ok(());
    }
    
    let request_line = lines[0];
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    
    if parts.len() < 2 {
        return Ok(());
    }
    
    let method = parts[0];
    let path = parts[1];
    
    // Route the request
    let response = match (method, path) {
        ("GET", "/") => {
            let html = get_index_html();
            create_response("200 OK", "text/html", &html)
        }
        ("GET", "/api/health") => {
            let health = get_health_json();
            create_response("200 OK", "application/json", &health)
        }
        ("GET", _) => {
            let not_found = get_not_found_html();
            create_response("404 Not Found", "text/html", &not_found)
        }
        _ => {
            let not_allowed = get_method_not_allowed_html();
            create_response("405 Method Not Allowed", "text/html", &not_allowed)
        }
    };
    
    // Send the response
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    
    Ok(())
}

/// Create an HTTP response string.
fn create_response(status: &str, content_type: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\nAccess-Control-Allow-Headers: content-type\r\n\r\n{}",
        status,
        content_type,
        body.len(),
        body
    )
}

/// Get the index HTML page.
fn get_index_html() -> String {
    r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Prism Web UI</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .container {
            background: white;
            border-radius: 12px;
            padding: 2rem;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            max-width: 600px;
            text-align: center;
        }
        h1 {
            color: #333;
            margin-bottom: 1rem;
            font-size: 2.5rem;
        }
        .subtitle {
            color: #666;
            margin-bottom: 2rem;
            font-size: 1.2rem;
        }
        .features {
            text-align: left;
            margin: 2rem 0;
        }
        .feature {
            padding: 0.5rem 0;
            border-bottom: 1px solid #eee;
        }
        .feature:last-child {
            border-bottom: none;
        }
        .status {
            background: #e8f5e8;
            color: #2d5a2d;
            padding: 1rem;
            border-radius: 8px;
            margin: 1rem 0;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🔮 Prism Web UI</h1>
        <p class="subtitle">Soroban Transaction Debugger</p>
        
        <div class="status">
            ✅ Server is running successfully!
        </div>

        <div class="features">
            <div class="feature">
                <strong>📊 Trace Visualization:</strong> Explore complex transaction traces in an interactive browser interface
            </div>
            <div class="feature">
                <strong>🔍 Error Analysis:</strong> Decode and understand Soroban transaction errors
            </div>
            <div class="feature">
                <strong>📈 Performance Profiling:</strong> Analyze resource consumption and execution patterns
            </div>
            <div class="feature">
                <strong>🔄 State Diffing:</strong> Compare contract state before and after transactions
            </div>
        </div>

        <p style="color: #888; margin-top: 2rem;">
            This is the basic Prism Web UI. Full web interface implementation coming soon!
        </p>
    </div>

    <script>
        // Basic health check
        fetch('/api/health')
            .then(response => response.json())
            .then(data => console.log('Health check:', data))
            .catch(err => console.error('Health check failed:', err));
    </script>
</body>
</html>
    "#.to_string()
}

/// Get the health check JSON response.
fn get_health_json() -> String {
    format!(
        r#"{{"status":"healthy","service":"prism-web-ui","version":"{}"}}"#,
        env!("CARGO_PKG_VERSION")
    )
}

/// Get the 404 Not Found HTML page.
fn get_not_found_html() -> String {
    r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 - Not Found</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .container {
            background: white;
            border-radius: 12px;
            padding: 2rem;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            max-width: 400px;
            text-align: center;
        }
        h1 {
            color: #333;
            margin-bottom: 1rem;
            font-size: 2rem;
        }
        p {
            color: #666;
            margin-bottom: 2rem;
        }
        a {
            color: #667eea;
            text-decoration: none;
            font-weight: bold;
        }
        a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>404 - Not Found</h1>
        <p>The page you're looking for doesn't exist.</p>
        <a href="/">Go back to Prism Web UI</a>
    </div>
</body>
</html>
    "#.to_string()
}

/// Get the 405 Method Not Allowed HTML page.
fn get_method_not_allowed_html() -> String {
    r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>405 - Method Not Allowed</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .container {
            background: white;
            border-radius: 12px;
            padding: 2rem;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            max-width: 400px;
            text-align: center;
        }
        h1 {
            color: #333;
            margin-bottom: 1rem;
            font-size: 2rem;
        }
        p {
            color: #666;
            margin-bottom: 2rem;
        }
        a {
            color: #667eea;
            text-decoration: none;
            font-weight: bold;
        }
        a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>405 - Method Not Allowed</h1>
        <p>The HTTP method is not allowed for this resource.</p>
        <a href="/">Go back to Prism Web UI</a>
    </div>
</body>
</html>
    "#.to_string()
}

/// Open the default browser to the specified URL.
fn open_browser(url: &str) -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn()?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()?;
    }
    
    Ok(())
}
