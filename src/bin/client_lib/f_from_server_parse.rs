use crate::client_lib::classes::c_client::Client;
use crate::client_lib::classes::e_text_color::ETextColor;
use crate::client_lib::cli_utils::f_print_utils::{print_cli};
use crate::client_lib::cli_utils::f_rusty_line_input::Printer;
use crate::shared_lib::c_command::Packet;
use crate::shared_lib::c_commands_solver::{CommandsSolver, ECommand};

pub async fn parse_from_server_commands(client: &mut Client, packet: Packet, solver: &mut CommandsSolver, printer: &mut Printer) -> ECommand {
    match packet.command {
        ECommand::GetUserId => {
            let id = packet.args[0].parse::<u32>().unwrap();
            client.set_id(id);
            print_cli(printer, "Connected to server", ETextColor::Green).await;

            let get_rooms_packet = CommandsSolver::create_command(ECommand::GetRooms, Vec::<String>::new());
            client.send_message(get_rooms_packet).await;
        }
        ECommand::GetRooms =>{
            print_cli(printer, "Available rooms list: ", ETextColor::White).await;
            print_cli(printer, packet.args[0].as_str(), ETextColor::White).await;
            if !client.is_in_room() {
                print_cli(printer, "/join_room [name] - to join room", ETextColor::White).await;
                print_cli(printer, "/create_room [name] [size] - to create own room", ETextColor::White).await;
            }
        }
        ECommand::Error => {
            print_cli(printer, packet.args[0].as_str(), ETextColor::Red).await;
        }
        ECommand::Info => {
            print_cli(printer, packet.args[0].as_str(), ETextColor::Green).await;
        }
        ECommand::JoinRoom => {
            client.set_in_room(true);
        }
        ECommand::UserMessage => {
            print_cli(printer, packet.args[0].as_str(), ETextColor::White).await;
        }
        ECommand::Disconnect => {
            if (client.is_connected()) {
                if (client.is_in_room()) {
                    client.send_message(Packet::new(ECommand::GetRooms, Vec::<String>::new())).await;
                    client.set_in_room(false);
                }
            }
        }
        _ => {}
    }


    return packet.command;
}
