use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use weui::{html_page_with_assets, theme_css};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");
    println!("Serving WeUI HTTP on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream);
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Ok(_) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..]);
        if let Some(path) = parse_path(&request) {
            match path {
                "/" | "/index.html" => serve_html(&mut stream),
                "/style.css" => serve_css(&mut stream),
                "/app.js" => serve_js(&mut stream),
                _ => serve_404(&mut stream),
            }
            return;
        }
    }
    serve_404(&mut stream);
}

fn parse_path(request: &str) -> Option<&str> {
    request.lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
}

fn serve_html(stream: &mut TcpStream) {
    let body = r#"
    <div class=\"weui-example\" style=\"padding: 24px;\">
        <h1>WeUI HTTP Example</h1>
        <button class=\"weui-btn weui-btn--primary\" id=\"primaryBtn\">Click me</button>
        <button class=\"weui-btn weui-btn--outline\" id=\"secondaryBtn\" style=\"margin-left: 12px;\">Secondary</button>
    </div>
    "#;

    let html = html_page_with_assets(
        "WeUI HTTP Example",
        body,
        Some("/style.css"),
        Some("/app.js"),
    );

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        html.len(),
        html
    );
    let _ = stream.write_all(response.as_bytes());
}

fn serve_css(stream: &mut TcpStream) {
    let css = theme_css();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/css; charset=UTF-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        css.len(),
        css
    );
    let _ = stream.write_all(response.as_bytes());
}

fn serve_js(stream: &mut TcpStream) {
    let js = app_js();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/javascript; charset=UTF-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        js.len(),
        js
    );
    let _ = stream.write_all(response.as_bytes());
}

fn serve_404(stream: &mut TcpStream) {
    let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
    let _ = stream.write_all(response.as_bytes());
}

fn app_js() -> &'static str {
    r#"document.addEventListener('DOMContentLoaded', function() {
    const primaryBtn = document.getElementById('primaryBtn');
    const secondaryBtn = document.getElementById('secondaryBtn');

    if (primaryBtn) {
        primaryBtn.addEventListener('click', function() {
            alert('Hello from WeUI Rust HTTP!');
        });
    }

    if (secondaryBtn) {
        secondaryBtn.addEventListener('click', function() {
            secondaryBtn.textContent = 'Clicked!';
        });
    }
});"#
}
