use crate::shared_lib::c_command::Packet;
use crossterm::style::Stylize;

pub fn log_input(user_id: u32, message: String) {
    let val = format!("<< {}:\n{}", user_id, message);
    println!("{}", val.green());
}

pub fn log_output(user_id: u32, packet: &Packet) {
    let val = format!(">> {}:\n{}", user_id, packet.to_string());
    println!("{}", val.yellow());
}
