use crate::client_lib::classes::c_client::Client;
use crate::client_lib::classes::e_text_color::ETextColor;
use crate::client_lib::cli_utils::f_print_utils::{clear_console, print_cli};
use crate::client_lib::cli_utils::f_rusty_line_input::Printer;
use crate::shared_lib::c_command::Packet;
use crate::shared_lib::c_commands_solver::{CommandsSolver, ECommand};
use crate::shared_lib::f_utils::get_time_stamp_str;

pub async fn parse_local_commands(client: &mut Client, line: String, solver: &mut CommandsSolver, printer: &mut Printer) -> ECommand {
    let packet = solver.pase_command_line(&line);

    let args = packet.args;

    match packet.command {
        ECommand::None => {
            if client.is_connected() {
                if client.is_in_room() {
                    client.send_message(Packet::new(ECommand::UserMessage, vec![line.clone()])).await;
                    let time_stamp = get_time_stamp_str();
                    print_cli(printer, format!("{} You: {}", time_stamp, line.clone()).as_str(), ETextColor::Yellow).await;
                    return packet.command;
                }
            }

            print_cli(printer, "Command not found use /help", ETextColor::Red).await;
        }
        ECommand::Connect => {
            if client.is_in_room() {
                print_cli(printer, "Disconnect from room first.", ETextColor::Red).await;
                return packet.command;
            }

            if client.is_connected() {
                client.disconnect().await;
            }

            if args.len() > 0 {
                connect_to_api(client, printer, args[0].as_str()).await;
            } else {
                connect_to_api(client, printer, client.get_default_address().as_str()).await;
            }
        }
        ECommand::Disconnect => {
            if client.is_connected() {
                if client.is_in_room() {
                    client.send_message(Packet::new(ECommand::Disconnect, vec![])).await;
                }else{
                    print_cli(printer, "Disconnected from server", ETextColor::Green).await;
                    client.disconnect().await;
                }
            }else{
                print_cli(printer, "You are not connected", ETextColor::Red).await;
            }
        }
        ECommand::ChangeName => {

            if (client.is_connected()){
                print_cli(printer, "Disconnect from server to change name", ETextColor::Red).await;
                return packet.command;
            }
            if args.len() == 0 {
                print_cli(printer, "Name not found", ETextColor::Red).await;
                return packet.command;
            }
            if args[0].as_str().to_string().trim().len() < 2 {
                print_cli(printer, "Name is empty or too short", ETextColor::Red).await;
                return packet.command;
            }

            client.change_name(args[0].as_str().to_string().trim());
            print_cli(printer, format!("Your name now is {}", args[0]).as_str(), ETextColor::Green).await;
        }
        ECommand::Help => {
            print_cli(printer, "Help:", ETextColor::Yellow).await;
            print_cli(printer, "/connect [empty/IP] - connecting to server by IP", ETextColor::White).await;
            print_cli(printer, "/disconnect - disconnecting from server", ETextColor::White).await;
            print_cli(printer, "/change_name [name] - change user name", ETextColor::White).await;
            print_cli(printer, "/help", ETextColor::White).await;
            print_cli(printer, "/get_rooms - get available chat rooms", ETextColor::White).await;
            print_cli(printer, "/create_room [name] [size]", ETextColor::White).await;
            print_cli(printer, "/join_room [name]", ETextColor::White).await;
            print_cli(printer, "/clear - flush console", ETextColor::White).await;
            print_cli(printer, "/quit", ETextColor::White).await;
        }
        ECommand::GetRooms => {
            if !rooms_commands_valid(client, printer).await {
                return packet.command;
            }
            if client.is_connected() {
                client.send_message(Packet::new(ECommand::GetRooms, Vec::<String>::new())).await;
            }
        }
        ECommand::CreateRoom => {
            if !rooms_commands_valid(client, printer).await {
                return packet.command;
            }

            if args.len() >= 2 {
                let room_name = args[0].trim();
                let size_parse = args[1].trim().parse::<u8>();

                let size = match size_parse {
                    Ok(size) => {
                        size
                    }
                    Err(_) => {
                        print_cli(printer, "Size is not valid", ETextColor::Red).await;
                        return packet.command;
                    }
                };

                if room_name.len() > 2 {
                    client.send_message(Packet::new(ECommand::CreateRoom, vec![room_name.to_string(), size.to_string()])).await;
                    return packet.command;
                }
            }

            print_cli(printer, "Not found valid arguments: [name] [size]", ETextColor::Red).await;
        }
        ECommand::JoinRoom => {
            if !rooms_commands_valid(client, printer).await {
                return packet.command;
            }

            if args.len() <= 0 {
                print_cli(printer, "Room name is not found in arguments.", ETextColor::Red).await;
            }else if args[0].trim().len() < 2 {
                print_cli(printer, "Name is empty or too short.", ETextColor::Red).await;
            }
            else{
                client.send_message(Packet::new(ECommand::JoinRoom, vec![args[0].to_string()])).await;
            }
        }
        ECommand::ClearCLI =>{
            clear_console();
        }
        _ => {}
    }
    packet.command
}


async fn rooms_commands_valid(client: &mut Client, printer: &mut Printer) -> bool {
    if !client.is_connected() {
        print_cli(printer, "Connect to server first.", ETextColor::Red).await;
        return false;
    }
    if client.is_connected() && client.is_in_room() {
        print_cli(printer, "Disconnect from room first.", ETextColor::Red).await;
        return false;
    }
    true
}

async fn connect_to_api(client: &mut Client, printer: &mut Printer, api: &str) {
    let val = client.connect(api).await;
    match val {
        Ok(_) => {
            print_cli(printer, format!("Connected to: {}", api).as_str(), ETextColor::Yellow).await;
        }
        Err(_) => {
            print_cli(printer, format!("Connection failed: {}", api).as_str(), ETextColor::Red).await;
        }
    }
}
