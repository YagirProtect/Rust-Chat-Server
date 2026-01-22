use crate::server_lib::c_hub::ServerClientsHub;
use crate::shared_lib::c_command::Packet;
use crate::shared_lib::c_commands_solver::{CommandsSolver, ECommand};
use std::sync::Arc;
use futures_util::future::err;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc::Sender;
use crate::client_lib::c_client::writer_loop;
use crate::server_lib::c_server_client::{EClientState, ServerClient, USERS_IDS_SOLVER};

mod shared_lib;
mod server_lib;
mod client_lib;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let hub = Arc::new(Mutex::new(ServerClientsHub::default()));


    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on 127.0.0.1:3000");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        let hub_for_task = Arc::clone(&hub);
        println!("client: {addr}");




        tokio::spawn(async move {
            let mut user_id = USERS_IDS_SOLVER.fetch_add(1, std::sync::atomic::Ordering::Release);
            let (rd, wr) = socket.into_split();

            let (out_tx, out_rx) = mpsc::channel::<String>(256);
            let (in_tx, mut in_rx) = mpsc::channel::<(String, bool)>(256);

            let reader =  tokio::spawn(async move {
                client_reader_loop(rd, in_tx).await;
            });
            let writer =  tokio::spawn(async move {
                writer_loop(wr, out_rx).await;
            });



            loop {
                let (line, ok) = match in_rx.recv().await {
                    Some(v) => v,
                    None => break,
                };

                if !ok {
                    break;
                }

                println!("in >> {}", line);

                let packet = Packet::load(line.as_str());

                let command = packet.command;
                let args = packet.args;
                match command {
                    ECommand::CreateUser => {
                        let user_name = args[0].clone();

                        {
                            let mut hub_guard = hub_for_task.lock().await;
                            let client = match hub_guard.create_user(user_name, user_id, out_tx.clone()){
                                Some(client) => client,
                                _ => { break;}
                            };

                            client.change_state(EClientState::Connected);
                        }
                        let packet = CommandsSolver::create_command(ECommand::GetUserId, [user_id.to_string()]);
                        send_message(out_tx.clone(), packet).await;
                    }
                    ECommand::GetRooms => {
                        {
                            let hub_guard = hub_for_task.lock().await;
                            let table = hub_guard.get_rooms_table();
                            let command = CommandsSolver::create_command(ECommand::GetRooms, [table]);
                            send_message(out_tx.clone(), command).await;
                        }
                    }
                    ECommand::CreateRoom => {

                        let room_name = args[0].clone();
                        let room_size = args[1].clone().parse::<u8>().unwrap();
                        {
                            let mut hub_guard = hub_for_task.lock().await;

                            let find_room = hub_guard.has_room(room_name.clone());


                            if (!find_room) {
                                hub_guard.create_room(room_name.clone(), room_size);
                            }else{
                                send_message(out_tx.clone(), Packet::new(ECommand::Error, vec!["Room with same name is exists".to_string()])).await;
                                continue;
                            }
                        }

                        join_room(out_tx.clone(), &hub_for_task, user_id, args).await;
                    }

                    ECommand::JoinRoom => {
                        join_room(out_tx.clone(), &hub_for_task, user_id, args).await;
                    }
                    ECommand::UserMessage => {
                        let mut hub_guard = hub_for_task.lock().await;
                        {
                            let sender = hub_guard.find_user_mut(user_id).unwrap().get_name();
                            let room = hub_guard.get_user_room(user_id);
                            println!("user message: {}", user_id);

                            match room {
                                None => {}
                                Some(r) => {
                                    let users = r.get_all_users_except_sender(user_id);
                                    for item in users {
                                        let user = match hub_guard.find_user_mut(item) {
                                            Some(user) => user,
                                            _ => { continue; }
                                        };

                                        println!("{:?}", user.get_id());

                                        user.send_message_to(args[0].clone(), sender.clone()).await;
                                    }
                                }
                            }
                        }
                    }
                    ECommand::Disconnect => {
                        let mut hub_guard = hub_for_task.lock().await;
                        hub_guard.disconnect_user(user_id);

                        let user = match hub_guard.find_user_mut(user_id) {
                            None => {
                                continue;
                            }
                            Some(user) => {
                                user
                            }
                        };
                        if (user.get_room_id() != -1) {

                            user.change_state(EClientState::Hub);
                            user.set_room_id(-1);
                        }


                        send_message(out_tx.clone(), Packet::new(ECommand::Info, vec!["Disconnected from room".to_string()])).await;
                        send_message(out_tx.clone(), Packet::new(ECommand::Disconnect, vec![])).await;
                    }
                    _ => {}
                }
            }

            let mut hub_guard = hub_for_task.lock().await;
            {
                hub_guard.disconnect_user(user_id);
                hub_guard.remove_user(user_id);
            }
            println!("disconnected: {addr}");
        });
    }
}

async fn join_room(out_tx: Sender<String>, hub_for_task: &Arc<Mutex<ServerClientsHub>>, user_id: u32, args: Vec<String>) {
    let room_name = args[0].clone();
    {
        let mut hub_guard = hub_for_task.lock().await;

        if (hub_guard.join_room(room_name.clone(), user_id)) {
            send_message(out_tx.clone(), Packet::new(ECommand::Info, vec![
                format!("Joined to room {}", room_name),
            ])).await;
            send_message(out_tx.clone(), Packet::new(ECommand::JoinRoom, vec![])).await;

        } else {
            send_message(out_tx, Packet::new(ECommand::Error, vec![
                "Room not exists or full".to_string(),
            ])).await;
        }
    }
}

async fn send_message(out_tx: Sender<String>, packet: Packet) {
    let val = format!("{}\n", packet.to_string());


    println!("out >> {}", val);

    if out_tx.send(val).await.is_err() {
        println!("error writing to socket");
    }
}


async fn client_reader_loop(rd: OwnedReadHalf, in_tx: mpsc::Sender<(String, bool)>) {
    let mut lines = BufReader::new(rd).lines();
    loop {
        match lines.next_line().await {
            Ok(Some(line)) => {
                if in_tx.send((line, true)).await.is_err() { break; }
            }
            Ok(None) => {
                let _ = in_tx.send((String::new(), false)).await;
                break;
            }
            Err(_) => {
                let _ = in_tx.send((String::new(), false)).await;
                break;
            }
        }
    }
}
