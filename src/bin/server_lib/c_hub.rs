use crate::server_lib::c_server_room::ServerRoom;
use crate::server_lib::c_server_client::ServerClient;

#[derive(Default)]
pub struct ServerClientsHub{
    users: Vec<ServerClient>,
    rooms: Vec<ServerRoom>,
}

impl ServerClientsHub {
    pub fn create_user(&mut self, name: String) -> &ServerClient {
        let user = ServerClient::new(name);

        self.users.push(user);

        let user = &self.users[self.users.len() - 1];

        user
    }
}

