use std::sync::Arc;
use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use crate::server_lib::c_hub::ServerClientsHub;
use crate::shared_lib::c_command::Packet;
use crate::shared_lib::c_commands_solver::{CommandsSolver, ECommand, ECommandType};

mod shared_lib;
mod server_lib;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let mut hub = Arc::new(Mutex::new(ServerClientsHub::default()));


    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on 127.0.0.1:3000");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        let hub_for_task = Arc::clone(&hub); // <-- ключевой момент
        println!("client: {addr}");

        let handle = tokio::spawn(async move {
            let mut buf = [0u8; 4096];
            let solver = CommandsSolver::default();
            let mut user_id = -1;

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("client 0");
                        break
                    },
                    Ok(n) => n,
                    Err(err) => {
                        println!("client disconnected {:?}", err);
                        break
                    },
                };


                let line: String = String::from_utf8_lossy(&buf[..n]).to_string();

                println!("in >> {}", line);

                let packet = Packet::load(line.as_str());

                let command = packet.command;
                let args = packet.args;
                match command {
                    ECommand::CreateUser => {
                        let user_name = args[0].clone();

                        let mut id: u32 = 0;

                        {
                            let mut hub_guard = hub_for_task.lock().await;
                            let client = hub_guard.create_user(user_name);

                            id = client.get_id();
                        }
                        user_id = id as i32;
                        let packet = CommandsSolver::create_command(ECommand::GetUserId, [id.to_string()]);
                        send_message(&mut socket, packet).await;

                    }
                    ECommand::GetRooms => {
                        {
                            let mut hub_guard = hub_for_task.lock().await;
                            let table = hub_guard.get_rooms_table();
                            let command = CommandsSolver::create_command(ECommand::GetRooms, [table]);
                            send_message(&mut socket, command).await;
                        }
                    }
                    ECommand::CreateRoom => {

                        let room_name = args[0].clone();
                        let room_size = args[1].clone().parse::<u8>().unwrap();
                        {
                            let mut hub_guard = hub_for_task.lock().await;
                            hub_guard.create_room(room_name.clone(), room_size);

                            if (hub_guard.join_room(room_name.clone(), user_id as u32)){
                                send_message(&mut socket, Packet::new(ECommand::Info, vec![
                                    format!("Joined to room {}", room_name),
                                ])).await;
                            }else{
                                send_message(&mut socket, Packet::new(ECommand::Error, vec![
                                    "Room not exists or full".to_string(),
                                ])).await;
                            }
                        }
                    }
                    _ => {}
                }
            }
            println!("disconnected: {addr}");
        });

    }
}

async fn send_message(socket:  &mut TcpStream, packet: Packet) {

    let val = format!("{}\n", packet.to_string());


    println!("out >> {}", val);
    
    if socket.write_all(val.as_bytes()).await.is_err() {
        println!("error writing to socket");
    }
}