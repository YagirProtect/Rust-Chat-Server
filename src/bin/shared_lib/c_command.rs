use serde::{Deserialize, Serialize};
use crate::shared_lib::c_commands_solver::ECommand;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Packet{
    pub command: ECommand,
    pub args: Vec<String>,
}

impl Packet {
    pub fn new(command: ECommand, args: Vec<String>) -> Self {
        Packet{command, args}
    }

    pub fn load(json: &str) -> Packet {
        let val: Packet = match serde_json::from_str(json) {
            Ok(json) => json,
            Err(e) => panic!("{}", e),
        };
        val
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}