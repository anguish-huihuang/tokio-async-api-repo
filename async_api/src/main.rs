use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").await.unwrap();

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            stream.read(&mut buffer).await.unwrap();
            println!("{}", String::from_utf8_lossy(&buffer));

            handle_connection(&mut stream).await;
        });
    }
}
//tcp
// async fn handle_connection(stream: &mut TcpStream) {
//     let content = "hellow";
//     stream.write_all(content.as_bytes()).await.unwrap();
//     stream.flush().await.unwrap();
// }
//http
async fn handle_connection(stream: &mut TcpStream) {
    let content = "hellow";
    stream.write_all(format!(
        "HTTP/1.1 200 OK\r\nContenr_length:{}\r\n\r\n{}",
        content.len(),
        content
    ).as_bytes(),).await.unwrap();
    stream.flush().await.unwrap();
}