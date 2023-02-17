use std::net::{TcpStream};


pub enum ConvState {
    Waiting,
    Going,
    Closed,
}

pub struct Client {
    pub id: Vec<u8>,
    pub stream: TcpStream,
}

pub struct Conversation {
    pub state: ConvState,
    pub client1: Client,
    pub client2: Option<Client>,
}

impl Conversation {
    pub fn new(client: Client) -> Conversation {
        Conversation {
            state: ConvState::Waiting,
            client1: client,
            client2: None,
        }
    }

    pub fn add_client(&mut self, client: Client) {
        self.client2 = Some(client);
        self.state = ConvState::Going;
    }

    pub fn get_id(&self) -> &Vec<u8> {
        &self.client1.id
    }
}