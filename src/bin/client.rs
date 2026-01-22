mod client_lib;
mod shared_lib;

use crate::client_lib::c_client::Client;
use crate::client_lib::cli_lib::print_utils::ETextColor::White;
use crate::client_lib::cli_lib::print_utils::{print_cli, ETextColor};
use crate::client_lib::cli_lib::rusty_line_input::spawn_rustyline_input;
use crate::client_lib::from_server_parse::parse_from_server_commands;
use crate::shared_lib::c_commands_solver::{CommandsSolver, ECommand};
use client_lib::from_local_client_parse::parse_local_commands;
use tokio::io;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let mut client = Client::new();

    let mut reader = client.take_incoming_rx();
    let mut solver = CommandsSolver::default();

    let (input_tx, mut input_rx) = mpsc::channel::<String>(64);
    let printer_rx = spawn_rustyline_input(input_tx);
    let mut printer = printer_rx
        .await
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "printer not created"))?;


    print_cli(&mut printer, "Welcome chat CLI", White).await;
    print_cli(&mut printer, "Use /help to see all the commands", White).await;

    loop {

        tokio::select! {
            Some(input) = input_rx.recv() => {
                let val = parse_local_commands(&mut client, input, &mut solver, &mut printer).await;
                if val == ECommand::Quit { break; }
            }


             ev = reader.recv() => {
                match ev {
                    Some(packet) => {
                        let val = parse_from_server_commands(&mut client, packet, &mut solver, &mut printer).await;
                    }
                    None => {
                        print_cli(&mut printer, "server/reader task closed channel", ETextColor::Red).await;
                        client.disconnect().await;
                    }
                }
            }
        }
    }
    Ok(())
}