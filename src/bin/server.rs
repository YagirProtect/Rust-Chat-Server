use std::sync::Arc;
use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use crate::server_lib::c_hub::ServerClientsHub;
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

                let (command, args) = solver.pase_command_line(&line, ECommandType::ToServer);

                match command {
                    ECommand::CreateUser => {
                        let user_name = args[0].clone();

                        let mut id: u32 = 0;

                        {
                            let mut hub_guard = hub_for_task.lock().await;
                            let client = hub_guard.create_user(user_name);

                            id = client.get_id();
                        }

                        println!("User created {id}");
                        let command = CommandsSolver::create_command(ECommand::GetUserId, id.to_string(), ECommandType::FromServer);
                        send_message(&mut socket, command).await;

                    }
                    _ => {}
                }


            }
            println!("disconnected: {addr}");
        });

    }
}

async fn send_message(socket:  &mut TcpStream, msg: String) {
    if socket.write_all(msg.as_bytes()).await.is_err() {
        println!("error writing to socket");
    }
}