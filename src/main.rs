use core::str;
use std::{io::Read, net::{TcpListener, TcpStream}};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'l', long = "listen", help = "start TCP listener instead of calling")]
    listen: bool,

    // host address
    host: String,

    // host port
    port: u16,
}

fn read_loop(mut stream: TcpStream) {
    println!("incoming connection: {}", stream.peer_addr().unwrap().ip());
    let mut buf = [0u8; 4096];
    loop {
        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break
                }
                let received = str::from_utf8(&buf[0..n]).unwrap();
                print!("{}", received)
            },
            Err(err) => println!("error: {}", err)
        }
    }
    println!("connection terminated!")
}

fn listen(host:String, port:u16) -> Result<(), std::io::Error> {
    let listen_address = format!("{address}:{port}", address=host, port=port);
    println!("listening on {}", listen_address);

    let tcp_listener = match TcpListener::bind(&listen_address) {
        Ok(tcp_listener) => tcp_listener,
        Err(err) => return Err(err),
    };

    for stream in tcp_listener.incoming() {
        let stream = match stream {
            Ok(conn) => conn,
            Err(err) => {
                println!("error on incoming connection: {}", err);
                continue;
            },
        };

        read_loop(stream);
        return Ok(())
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if cli.listen {
        match listen(cli.host, cli.port) {
            Ok(()) => (),
            Err(err) => println!("error setting up listener: {}", err)
        }
    }
}
