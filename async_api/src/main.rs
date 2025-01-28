use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("HTTP服务器启动，监听 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // 读取客户端发送的数据
            let n = match socket.read(&mut buf).await {
                Ok(0) => return, // 客户端关闭连接
                Ok(n) => n,
                Err(e) => {
                    eprintln!("读取错误：{}", e);
                    return;
                }
            };

            // 将读取到的字节数据转换为字符串
            let request = String::from_utf8_lossy(&buf[0..n]);
            println!("收到请求：{}", request);

            // 检查是否是GET请求
            if let Some(request_line) = request.lines().next() {
                if request_line.starts_with("GET / HTTP/1.1") {
                    // 返回HTTP响应
                    let response = "HTTP/1.1 200 OK\r\n\
                        Content-Type: text/html\r\n\r\n\
                        <html><body><h1>Holl!</h1></body></html>";
                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                        eprintln!("写入错误：{}", e);
                        return;
                    }
                } else {
                    // 返回404响应
                    let response = "HTTP/1.1 404 Not Found\r\n\
                        Content-Type: text/html\r\n\r\n\
                        <html><body><h1>404 Not Found</h1></body></html>";
                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                        eprintln!("写入错误：{}", e);
                        return;
                    }
                }
            }
        });
    }
}