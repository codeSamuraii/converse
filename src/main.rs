use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use converse::{Client, Conversation, ConvState};


fn get_client(mut stream: TcpStream) -> Result<Client, std::io::Error> {
    let mut data = String::with_capacity(16);
    match stream.read_to_string(&mut data) {
        Ok(_) => { println!("Client ID: {:?}", data); },
        Err(e) => { println!("Error: {}", e); return Err(e); }
    }

    let client = Client {
        conv_id: data,
        stream: stream,
    };

    return Ok(client);
}


fn add_to_conversation(client: Client, conversations: &mut Vec<Conversation>) {
    for conv in conversations.iter_mut() {
        if conv.id == client.conv_id {
            println!("Adding client to conversation: {:?}", conv.id);
            conv.add_client(client);
            return;
        }
    }

    let conv = Conversation::new(client);
    println!("Created new conversation: {:?}", conv.id);
    conversations.push(conv);
}


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3443")?;
    let mut conversations: Vec<Conversation> = Vec::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client = get_client(stream).unwrap();
                add_to_conversation(client, &mut conversations)
            }
            Err(e) => { println!("Error: {}", e); }
        }
    }

    Ok(())
}