use std::{
    io::{self, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
};
mod dir;
use dir::dir_reader::list_files;

fn main() -> io::Result<()> {
    let _ = open_conn();
    Ok(())
}

fn open_conn() -> io::Result<()> {
    let addr: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket: SocketAddrV4 = SocketAddrV4::new(addr, 8080);
    let listener: TcpListener = TcpListener::bind(socket)?;
    let files = list_files()?;

    for stream in listener.incoming() {
        match stream {
            Ok(st) => {
                _ = handle(st, &files);
            }
            Err(_) => {
                println!("An error has occured");
            }
        };
    }
    Ok(())
}

fn handle(mut stream: TcpStream, files: &str) -> io::Result<()> {
    let msg = format!("<h3>Directories:</h3>\n<div>{}</div>", files);

    let lines: &[String] = &[
        format!("HTTP/1.1 200 OK"),
        format!("Content-Length: {}", msg.len()),
        format!("Content-Type: text/html"),
        format!(""),
        msg.to_string(),
    ];
    stream.write_all(lines.join("\n").as_bytes())?;
    Ok(())
}
