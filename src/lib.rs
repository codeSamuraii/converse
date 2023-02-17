use std::net::{TcpStream};

pub enum ConvState {
    Waiting,
    Going,
    Closed,
}

pub struct Client {
    pub conv_id: String,
    pub stream: TcpStream,
}

pub struct Conversation {
    pub id: String,
    pub state: ConvState,
    pub client1: Client,
    pub client2: Option<Client>,
}

impl Conversation {
    pub fn new(client: Client) -> Conversation {
        Conversation {
            id: client.conv_id.clone(),
            state: ConvState::Waiting,
            client1: client,
            client2: None,
        }
    }

    pub fn add_client(&mut self, client: Client) {
        self.client2 = Some(client);
        self.state = ConvState::Going;
    }
}