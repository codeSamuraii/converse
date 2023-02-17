use std::net::{TcpStream};


pub enum ConvState {
    Waiting,
    Going,
    Closed,
}

pub struct Client<Copy> {
    pub id: Vec<u8>,
    pub stream: TcpStream,
}

pub struct Conversation {
    pub state: ConvState,
    pub client1: Client,
    pub client2: Option<Client>,
}