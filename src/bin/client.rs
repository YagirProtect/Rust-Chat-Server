mod client_lib;
mod shared_lib;

use crate::client_lib::c_client::Client;
use crate::shared_lib::c_commands_solver::{CommandsSolver, ECommand, ECommandType};
use std::io::BufRead;
use std::num::ParseIntError;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use crate::shared_lib::c_command::Packet;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let mut client = Client::new();

    let mut reader = client.take_incoming_rx();

    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();
    let mut line = String::new();
    let mut solver = CommandsSolver::default();


    loop {

        tokio::select! {
            line = stdin.next_line() => {
                let Some(input) = line? else {break;};
                let input = input.trim();
                if (input.is_empty()){
                    continue;
                }
                let val = parse_local_commands(&mut client, input.to_string(), &mut solver).await;

                if (val == ECommand::Quit){
                    break;
                }
            }


             ev = reader.recv() => {
                match ev {
                    Some(packet) => {
                        let val = parse_from_server_commands(&mut client, packet, &mut solver).await;
                    }
                    None => {
                        println!("server/reader task closed channel");
                        client.disconnect().await;
                    }
                }
            }
        }
    }
    Ok(())
}

async fn parse_local_commands(client: &mut Client, line: String, solver: &mut CommandsSolver) -> ECommand {
    let packet = solver.pase_command_line(&line);

    let args = packet.args;

    match packet.command {
        ECommand::Connect => {
            if (client.is_connected()) {
                client.disconnect().await;
            }

            if (args.len() > 0) {
                let val = client.connect(args[0].as_str()).await;
                match val {
                    Ok(_) => {}
                    Err(_) => { println!("Connection failed: {}", args[0].as_str()); }
                }
            } else {
                let val = client.connect("127.0.0.1:3000").await;
                match val {
                    Ok(_) => {}
                    Err(_) => { println!("Connection failed 127.0.0.1:3000"); }
                }
            }
        }
        ECommand::Disconnect => {
            if (client.is_connected()) {
                client.disconnect().await;
            }
        }
        ECommand::ChangeName => {
            if (client.is_connected()) {
                println!("disconnect to change name");
                return packet.command;
            }

            if (args.len() < 0) {
                println!("name not found");
                return packet.command;
            }
            if (args[0].as_str().to_string().trim().len() > 2) {
                println!("name is too short");
                return packet.command;
            }

            client.change_name(args[0].as_str().to_string().trim());
        }
        ECommand::Help => {}
        ECommand::Quit => {}
        ECommand::GetRooms => {
            if (client.is_connected()) {
                client.send_message(Packet::new(ECommand::GetRooms, Vec::<String>::new())).await;
            }else{
                println!("not connected");
            }
        }
        ECommand::CreateRoom => {
            if (args.len() >= 2) {
                let room_name = args[0].trim();


                let size_parse = args[1].trim().parse::<u8>();

                let size = match size_parse {
                    Ok(size) => {
                        size
                    }
                    Err(_) => {
                        println!("room size error");
                        return packet.command;
                    }
                };

                if (room_name.len() > 2) {
                    client.send_message(Packet::new(ECommand::CreateRoom, vec![room_name.to_string(), size.to_string()])).await;
                    return packet.command;
                }
            }

            println!("room name error");
        }
        _ => {}
    }


    return packet.command;
}

async fn parse_from_server_commands(client: &mut Client, packet: Packet, solver: &mut CommandsSolver) -> ECommand {
    match packet.command {
        ECommand::GetUserId => {
            let id = packet.args[0].parse::<u32>().unwrap();
            client.set_id(id);
            println!("User Authorized");

            let get_rooms_packet = CommandsSolver::create_command(ECommand::GetRooms, Vec::<String>::new());

            client.send_message(get_rooms_packet).await;
        }
        ECommand::GetRooms =>{
            println!("{}", packet.args[0]);
        }
        ECommand::Error => {
            println!("Error: {}", packet.args[0]);
        }
        ECommand::Info => {
            println!("Info: {}", packet.args[0]);
        }
        _ => {}
    }


    return packet.command;
}
