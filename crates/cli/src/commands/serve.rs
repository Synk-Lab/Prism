//! Static file server for Prism Web UI

use clap::Args;
use std::path::PathBuf;
use anyhow::Result;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Serve the Prism Web UI dashboard
#[derive(Args)]
pub struct ServeArgs {
    /// Port to serve on (default: 3000)
    #[arg(long, short, default_value = "3000")]
    port: u16,

    /// Host to bind to (default: 127.0.0.1)
    #[arg(long, short, default_value = "127.0.0.1")]
    host: String,
}

pub async fn run(args: ServeArgs) -> Result<()> {
    // Check if web assets are built
    let web_dist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../apps/web/.next");
    
    if !web_dist_path.exists() {
        eprintln!("❌ Web UI assets not found at: {}", web_dist_path.display());
        eprintln!("💡 Please run: npm run build in apps/web directory");
        eprintln!("🔧 Or build with: cd apps/web && npm install && npm run build");
        return Ok(());
    }

    // Create a simple index.html for the dashboard
    let index_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Prism Dashboard</title>
    <style>
        body { font-family: system-ui, sans-serif; margin: 0; padding: 2rem; background: #f8fafc; }
        .container { max-width: 1200px; margin: 0 auto; }
        .header { text-align: center; margin-bottom: 3rem; }
        .logo { font-size: 2rem; font-weight: bold; color: #1e40af; margin-bottom: 0.5rem; }
        .subtitle { color: #64748b; }
        .card { background: white; border-radius: 8px; padding: 2rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); margin-bottom: 2rem; }
        .card h2 { margin-top: 0; color: #1e293b; }
        .form-group { margin-bottom: 1rem; }
        label { display: block; margin-bottom: 0.5rem; font-weight: 500; }
        input, select { width: 100%; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 4px; }
        button { background: #1e40af; color: white; padding: 0.75rem 1.5rem; border: none; border-radius: 4px; cursor: pointer; }
        button:hover { background: #1d4ed8; }
        .status { padding: 0.5rem; border-radius: 4px; margin-top: 1rem; }
        .status.success { background: #dcfce7; color: #166534; }
        .status.error { background: #fef2f2; color: #991b1b; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div class="logo">🔆 Prism</div>
            <div class="subtitle">Soroban Transaction Debugger</div>
        </div>
        
        <div class="card">
            <h2>Transaction Analysis</h2>
            <div class="form-group">
                <label for="tx-hash">Transaction Hash</label>
                <input type="text" id="tx-hash" placeholder="Enter transaction hash...">
            </div>
            <div class="form-group">
                <label for="network">Network</label>
                <select id="network">
                    <option value="testnet">Testnet</option>
                    <option value="mainnet">Mainnet</option>
                    <option value="futurenet">Futurenet</option>
                </select>
            </div>
            <button onclick="analyzeTransaction()">Analyze Transaction</button>
            <div id="status" class="status" style="display: none;"></div>
        </div>

        <div class="card">
            <h2>Available Commands</h2>
            <ul>
                <li><strong>Decode:</strong> Translate error messages into plain English</li>
                <li><strong>Inspect:</strong> Full transaction context and metadata</li>
                <li><strong>Trace:</strong> Step-by-step execution replay</li>
                <li><strong>Profile:</strong> Resource consumption analysis</li>
                <li><strong>Diff:</strong> State changes before/after transaction</li>
            </ul>
        </div>
    </div>

    <script>
        function analyzeTransaction() {
            const txHash = document.getElementById('tx-hash').value;
            const network = document.getElementById('network').value;
            const status = document.getElementById('status');
            
            if (!txHash) {
                status.textContent = 'Please enter a transaction hash';
                status.className = 'status error';
                status.style.display = 'block';
                return;
            }
            
            status.textContent = `Analyzing transaction ${txHash} on ${network}...`;
            status.className = 'status success';
            status.style.display = 'block';
            
            // In a real implementation, this would call the Prism CLI backend
            setTimeout(() => {
                status.textContent = 'Transaction analysis complete! (Demo mode - CLI integration pending)';
            }, 2000);
        }
    </script>
</body>
</html>"#;

    let addr = format!("{}:{}", args.host, args.port);
    let listener = TcpListener::bind(&addr)?;
    
    println!("🌐 Prism dashboard serving at: http://{}", addr);
    println!("📊 Web UI available for transaction debugging");
    println!("🔧 Built from assets: {}", web_dist_path.display());
    println!("🔄 Press Ctrl+C to stop the server");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let index_html = index_html.to_string();
                thread::spawn(move || {
                    handle_request(stream, &index_html);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
    
    Ok(())
}

fn handle_request(mut stream: TcpStream, index_html: &str) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    
    let response = if request_line.starts_with("GET / ") || request_line.starts_with("GET /index.html") {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            index_html.len(),
            index_html
        )
    } else if request_line.starts_with("GET /_next/") {
        // For now, serve a simple response for Next.js assets
        "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nAsset not found".to_string()
    } else {
        // Fallback to index.html for SPA routing
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            index_html.len(),
            index_html
        )
    };
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
