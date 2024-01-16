use damn_core::resolve_connection;
use std::io::{Read, Write};
use std::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut connection = resolve_connection("jabber.org").await?;
    // let mut connection = TcpStream::connect("88.99.233.240:5222").await.unwrap();
    connection.writable().await?;

    connection
        .write_all("Hello XMPP Server! I am writing my own client".as_bytes())
        .await?;

    connection.readable().await?;
    let mut buffer = String::new();
    connection.read_to_string(&mut buffer).await?;
    // let size = connection.read(&mut buffer).await?;
    // println!("{size}");
    // let resp = std::str::from_utf8(&buffer[..size]).unwrap();
    println!("{buffer}");
    Ok(())

    // let mut connection = TcpStream::connect("88.99.233.240:5222").unwrap();

    // connection
    //     .write_all(
    //         "GET / HTTP/1.1\r\nHost: chapril.org:5222\nUser-Agent: curl/8.4.0\nAccept: */*\n"
    //             .as_bytes(),
    //     )
    //     .unwrap();

    // let mut buffer = Vec::with_capacity(1024);
    // let size = connection.read_to_end(&mut buffer).unwrap();
    // println!("{size}");
    // let resp = std::str::from_utf8(&buffer[..size]).unwrap();
    // println!("{resp}");
    // Ok(())
}
