use std::io::{Read, Error};
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

impl Client {
    pub fn from_stream(mut stream: TcpStream) -> Result<Client, Error> {
        let mut data = String::with_capacity(16);
        match stream.read_to_string(&mut data) {
            Ok(_) => { println!("New client for ID {:?}", data); },
            Err(e) => { println!("Error: {}", e); return Err(e); }
        }

        let client = Client {
            conv_id: data,
            stream: stream,
        };

        return Ok(client)
    }
}

pub struct Session {
    pub id: String,
    pub state: ConvState,
    pub client1: Client,
    pub client2: Option<Client>,
}

impl Session {
    pub fn new(client: Client) -> Session {
        Session {
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

pub struct SessionManager {
    pub sessions: Vec<Session>
}

impl SessionManager {
    pub fn initialize() -> SessionManager {
        SessionManager {
            sessions: Vec::new()
        }
    }

    pub fn add_client(&mut self, client: Client) {
        // Look for an existing matching session
        for session in self.sessions.iter_mut() {
            if session.id == client.conv_id {
                println!("Adding client to session: {:?}", session.id);
                session.add_client(client);
                return;
            }
        }

        // Create one if it doesn't exist
        let session = Session::new(client);
        println!("Creating session {:?}", session.id);
        self.sessions.push(session);
    }
}