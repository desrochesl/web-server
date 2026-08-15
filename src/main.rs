use std::{
    io::{self, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
};
mod dir;
mod requests;
use dir::dir_reader::list_files;
use requests::requests::{HTML, HTTPRequest};

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
            Err(e) => {
                println!("An error has occured, {:?}", e);
            }
        };
    }
    Ok(())
}

fn handle(mut stream: TcpStream, files: &str) -> io::Result<()> {
    let msg: HTML = HTML {
        tes: format!("<h3>Directories:</h3>\n<div>{}</div>", files),
    };
    let request: HTTPRequest = HTTPRequest {
        req_type: String::from("200"),
        msg: msg,
    };
    stream.write_all(format!("{}", request).as_bytes())?;
    Ok(())
}
