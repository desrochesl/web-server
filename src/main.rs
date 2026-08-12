use std::{
    io::{self, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
};

mod dir;

use dir::dir_reader;

fn main() -> io::Result<()> {
    let files = dir_reader()?;
    for file in files {
        println!("{:?}", file.as_path());
    }
    Ok(())
}

fn open_conn() -> io::Result<()> {
    let addr: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket: SocketAddrV4 = SocketAddrV4::new(addr, 8080);
    let listener: TcpListener = TcpListener::bind(socket)?;

    for stream in listener.incoming() {
        match stream {
            Ok(st) => {
                _ = handle(st);
            }
            Err(_) => {
                println!("An error has occured");
            }
        };
    }

    Ok(())
}

fn handle(mut stream: TcpStream) -> io::Result<()> {
    let msg: &str = "<h2>Dir:</h2>
                    <p>file_name.ext</p>";

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
