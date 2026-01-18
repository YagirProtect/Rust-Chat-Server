use std::collections::HashMap;
pub enum ECommandType{
    User,
    ToServer,
    FromServer
}


#[derive(PartialEq, Default, Ord, PartialOrd, Eq, Hash, Copy, Clone, Debug)]
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



    GetUserId
}

impl ECommand {

    pub fn get_user_pattern() -> HashMap<ECommand, Vec<String>>  {
        let mut hash_map = HashMap::new();

        hash_map.insert(ECommand::None, vec![]);
        hash_map.insert(ECommand::Connect, vec!["/connect".to_string(), "/c".to_string()]);
        hash_map.insert(ECommand::Disconnect, vec!["/disconnect".to_string()]);
        hash_map.insert(ECommand::ChangeName, vec!["/changename".to_string()]);
        hash_map.insert(ECommand::Help, vec!["/help".to_string()]);
        hash_map.insert(ECommand::Quit, vec!["/quit".to_string(), "/q".to_string()]);


        hash_map
    }

    pub fn get_to_server_pattern() -> HashMap<ECommand, Vec<String>>  {
        let mut hash_map = HashMap::new();

        hash_map.insert(ECommand::CreateUser, vec!["/create_user".to_string()]);

        hash_map
    }
    pub fn get_from_server_pattern() -> HashMap<ECommand, Vec<String>>  {
        let mut hash_map = HashMap::new();

        hash_map.insert(ECommand::GetUserId, vec!["/get_user_id".to_string()]);
        hash_map.insert(ECommand::GetRooms, vec!["/get_rooms".to_string()]);

        hash_map
    }
}

#[derive(Default)]
pub struct CommandsSolver{}

impl CommandsSolver {
    pub fn create_command(command: ECommand, args: String, user_type: ECommandType) -> String {
        let commands = match user_type {
            ECommandType::User => {
                ECommand::get_user_pattern()
            }
            ECommandType::ToServer => {
                ECommand::get_to_server_pattern()
            }
            ECommandType::FromServer => {
                ECommand::get_from_server_pattern()
            }
        };


        let val = &commands[&command];


        return format!("{} {}", val[0], args);
    }
}

impl CommandsSolver {
    pub fn pase_command_line(&self, line: &String, user_type: ECommandType) -> (ECommand, Vec<String>) {

        let command = line.trim();

        let commands = match user_type {
            ECommandType::User => {
                ECommand::get_user_pattern()
            }
            ECommandType::ToServer => {
                ECommand::get_to_server_pattern()
            }
            ECommandType::FromServer => {
                ECommand::get_from_server_pattern()
            }
        };

        let mut result = ECommand::None;

        for (i, (c, strs)) in commands.iter().enumerate() {
            for str in strs {
                if (command.starts_with(str)){
                    result = *c;
                    break;
                }
            }
            if (result != ECommand::None){
                break;
            }
        }

        if (result != ECommand::None) {
            let mut parts_owned: Vec<String> = command
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

            parts_owned.remove(0);

            return (result, parts_owned);
        }
        (result, vec![])
    }
}