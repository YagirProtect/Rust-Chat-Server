use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::shared_lib::c_command::Packet;

pub enum ECommandType{
    User,
    ToServer,
    FromServer
}


#[derive(PartialEq, Default, Ord, PartialOrd, Eq, Hash, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ECommand{
    #[default]
    None,
    Connect,
    Disconnect,
    ChangeName,
    Help,
    Quit,



    CreateUser,
    GetRooms,
    CreateRoom,
    JoinRoom,


    GetUserId,
    Error,
    Info,
}

impl ECommand {

    pub fn user_available_commands() -> Vec<ECommand>{
        vec![
            ECommand::Connect,
            ECommand::Disconnect,
            ECommand::ChangeName,
            ECommand::Help,
            ECommand::Quit,
            ECommand::GetRooms
        ]
    }

    pub fn get_commands_strings() -> HashMap<ECommand, Vec<String>>  {
        let mut hash_map = HashMap::new();

        hash_map.insert(ECommand::None, vec![]);
        hash_map.insert(ECommand::Disconnect, vec!["/disconnect".to_string()]);
        hash_map.insert(ECommand::ChangeName, vec!["/changename".to_string()]);
        hash_map.insert(ECommand::Help, vec!["/help".to_string()]);
        hash_map.insert(ECommand::CreateUser, vec!["/create_user".to_string()]);
        hash_map.insert(ECommand::GetRooms, vec!["/get_rooms".to_string()]);
        hash_map.insert(ECommand::CreateRoom, vec!["/create_room".to_string()]);
        hash_map.insert(ECommand::GetUserId, vec!["/get_user_id".to_string()]);
        hash_map.insert(ECommand::Connect, vec!["/connect".to_string(), "/cnt".to_string()]);
        hash_map.insert(ECommand::Quit, vec!["/quit".to_string(), "/q".to_string()]);

        hash_map
    }
}

#[derive(Default)]
pub struct CommandsSolver{}

impl CommandsSolver {
    pub fn create_command<I, S>(command: ECommand, args: I) -> Packet
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Packet::new(command, args.into_iter().map(Into::into).collect())
    }
}

impl CommandsSolver {
    pub fn pase_command_line(&self, line: &String) -> Packet {
        let command = line.trim();

        let commands = ECommand::get_commands_strings();

        let mut result = ECommand::None;

        for (i, (c, strs)) in commands.iter().enumerate() {
            for str in strs {
                if (command.starts_with(str)) {
                    result = *c;
                    break;
                }
            }
            if (result != ECommand::None) {
                break;
            }
        }

        if (result != ECommand::None) {
            let mut parts_owned: Vec<String> = command
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            parts_owned.remove(0);

            return Packet::new(result, parts_owned);
        }

        Packet::new(ECommand::None, vec![])
    }
}