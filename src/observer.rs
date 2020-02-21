use super::client::Client;

pub struct Observer {
    clients: Vec<Client>
}

impl Observer {
    pub fn new() -> Observer {
        Observer {
            clients: Vec::new()
        }
    }
    pub fn add(&mut self, mut client: Client) {
        client.run();
        self.clients.push(client);
    }
}
