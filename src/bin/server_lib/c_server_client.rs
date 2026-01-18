use crate::server_lib::c_server_client::EClientState::Connected;
use std::sync::atomic::AtomicU32;

pub enum EClientState{
    Connected,
    Hub,
    InRoom
}

pub static USERS_IDS_SOLVER: AtomicU32 = AtomicU32::new(0);

pub struct ServerClient{
    user_id: u32,
    name: String,
    state: EClientState
}


impl ServerClient {
    pub fn new(user_name: String) -> ServerClient {
        ServerClient {
            user_id: USERS_IDS_SOLVER.fetch_add(1, std::sync::atomic::Ordering::Release),
            name: user_name,
            state: Connected
        }

    }

    pub fn get_id(&self) -> u32 {
        self.user_id
    }
}