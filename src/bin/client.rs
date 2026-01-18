mod client_lib;
mod shared_lib;

use crate::client_lib::c_client::Client;
use std::io;
use std::io::{BufRead, BufReader};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::shared_lib::c_commands_solver::{CommandsSolver, ECommand, ECommandType};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let mut client = Client::new();

    let mut stdin = BufReader::new(io::stdin());
    let mut line = String::new();
    let mut solver = CommandsSolver::default();

    loop {
        line.clear();
        let line_size = stdin.read_line(&mut line)?;
        if (line_size == 0){
            break;
        }


        line = line.trim().to_string();

        if (line.len() == 0){
            continue;
        }


        let (command, args) = solver.pase_command_line(&line, ECommandType::User);

        match command {
            ECommand::Connect => {
                if (client.is_connected()){
                    client.disconnect().await;
                }

                if (args.len() > 0) {
                    let val = client.connect(args[0].as_str()).await;
                    match val {
                        Ok(_) => {}
                        Err(_) => {println!("Connection failed: {}", args[0].as_str());}
                    }
                }else{
                    let val = client.connect("127.0.0.1:3000").await;
                    match val {
                        Ok(_) => {}
                        Err(_) => {println!("Connection failed 127.0.0.1:3000");}
                    }
                }
            }
            ECommand::Disconnect => {
                if (client.is_connected()) {
                    client.disconnect().await;
                }
            }
            ECommand::ChangeName => {

                if (client.is_connected()){
                    println!("disconnect to change name");
                    continue;
                }

                if (args.len() < 0) {
                    println!("name not found");
                    continue;
                }
                if (args[0].as_str().to_string().trim().len() > 2) {
                    println!("name is too short");
                    continue;
                }

                client.change_name(args[0].as_str().to_string().trim());
            }
            ECommand::Help => {}
            ECommand::Quit => {
                break;
            }

            _ => {}
        }
    }
    Ok(())
}
