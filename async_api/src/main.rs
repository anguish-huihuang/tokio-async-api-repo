use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

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
                // 解析请求路径
                let path = request_line.split_whitespace().nth(1).unwrap_or("/");
                println!("请求路径：{}", path);

                // 根据路径返回不同的响应
                let response = match path {
                    "/apple" => {
                        "HTTP/1.1 200 OK\r\n\
                        Content-Type: text/html\r\n\r\n\
                        <html><body><h1>Apples are delicious!</h1></body></html>"
                    },
                    "/banana" => {
                        "HTTP/1.1 200 OK\r\n\
                        Content-Type: text/html\r\n\r\n\
                        <html><body><h1>Bananas are great too!</h1></body></html>"
                    },
                    _ => {
                        "HTTP/1.1 404 Not Found\r\n\
                        Content-Type: text/html\r\n\r\n\
                        <html><body><h1>404 Not Found</h1></body></html>"
                    }
                };

                // 发送响应
                if let Err(e) = socket.write_all(response.as_bytes()).await {
                    eprintln!("写入错误：{}", e);
                    return;
                }
            }
        });
    }
}