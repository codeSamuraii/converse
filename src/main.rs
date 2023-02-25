use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use converse::{Client, SessionManager};


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3443")?;
    let mut session_manager = SessionManager::initialize();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client = Client::from_stream(stream).unwrap();
                session_manager.add_client(client)
            }
            Err(e) => { println!("Error: {}", e); }
        }
    }

    Ok(())
}