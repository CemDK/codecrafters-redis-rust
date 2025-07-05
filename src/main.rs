use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("new client {:?}", addr);

                tokio::spawn(async move {
                    handle_connection(socket).await;
                });
            }
            Err(e) => println!("Couldn't get client {:?}", e),
        }
    }
}

async fn handle_connection(mut socket: TcpStream) {
    println!("handling incoming connection");

    let mut buf = [0; 1024];
    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                println!("Connection closed by client");
                return;
            }
            Ok(n) => {
                println!("Read {} bytes", n);
            }
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };

        if let Err(e) = socket.write_all("+PONG\r\n".as_bytes()).await {
            eprintln!("failed to write to socket; err = {:?}", e);
            return;
        }
    }
}
