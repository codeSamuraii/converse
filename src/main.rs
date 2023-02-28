use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use converse::{Client, SessionManager};
use std::thread;


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3443")?;
    let mut session_manager = SessionManager::initialize();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client = Client::from_stream(stream).unwrap();
                let session = session_manager.add_client(client);
                thread::spawn(move || {
                    session.process_data();
                });
            }
            Err(e) => { println!("Error: {}", e); }
        }
    }

    Ok(())
}