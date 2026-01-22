use crate::server_lib::c_server_client::EClientState::Connected;
use std::sync::atomic::AtomicU32;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use crate::shared_lib::c_command::Packet;
use crate::shared_lib::c_commands_solver::ECommand;
use crate::shared_lib::c_commands_solver::ECommand::UserMessage;
use crate::shared_lib::utils::get_time_stamp_str;

#[derive(Debug)]
pub enum EClientState{
    Connected,
    Hub,
    InRoom
}

pub static USERS_IDS_SOLVER: AtomicU32 = AtomicU32::new(0);

#[derive(Debug)]
pub struct ServerClient {
    user_id: u32,
    room_id: i32,
    name: String,
    state: EClientState,
    sender: Sender<String>,
}
impl ServerClient {
    pub fn new(user_name: String, user_id: u32, sender: Sender<String>) -> ServerClient {


        ServerClient {
            user_id: user_id,
            name: user_name,
            state: Connected,
            room_id: -1,
            sender: sender
        }
    }


    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub async fn send_message_to(&self, msg: String, sender_name: String) {
        let time_stamp = get_time_stamp_str();

        let packet = Packet::new(ECommand::UserMessage, vec![format!("{} {}: {}", time_stamp, sender_name, msg)]);

        if (self.sender.send(packet.to_string()).await.is_err()){
            println!("Send message to: {} error", self.user_id);
        }
    }



    pub fn change_state(&mut self, client_state: EClientState){
        self.state = client_state;
    }
    pub fn get_id(&self) -> u32 {
        self.user_id
    }
    pub fn get_room_id(&self) -> i32 {
        self.room_id
    }

    pub fn set_in_hub(&mut self){
        self.room_id = -1;
        self.change_state(EClientState::Hub);
    }

    pub fn set_room_id(&mut self, room_id: i32) {
        self.room_id = room_id;
        self.change_state(EClientState::InRoom);
    }
}