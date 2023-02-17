use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use converse::{Client, Conversation, ConvState};


fn get_client(mut stream: TcpStream) -> Result<Client> {
    let mut data: Vec<u8> = Vec::new();
    match stream.read(&mut data) {
        Ok(_) => { println!("Client ID: {:?}", data); },
        Err(e) => { println!("Error: {}", e); return Err(e); }
    }

    let client = Client {
        id: data,
        stream: stream,
    };

    Ok(client);
}


fn get_conversation() {

}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3443")?;
    let mut conversations: Vec<Conversation> = Vec::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client = get_client(stream);
            }
            Err(e) => { println!("Error: {}", e); }
        }
    }

    Ok(())
}