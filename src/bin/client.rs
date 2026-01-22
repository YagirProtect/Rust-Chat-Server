mod client_lib;
mod shared_lib;

use client_lib::classes::c_client::Client;
use client_lib::cli_utils::f_print_utils::{print_cli};
use client_lib::cli_utils::f_rusty_line_input::spawn_rustyline_input;
use crate::client_lib::f_from_server_parse::parse_from_server_commands;
use crate::shared_lib::c_commands_solver::{CommandsSolver, ECommand};
use client_lib::f_from_local_client_parse::parse_local_commands;
use tokio::io;
use tokio::sync::mpsc;
use shared_lib::c_config::Config;
use crate::client_lib::classes::e_text_color::ETextColor;
use crate::client_lib::cli_utils::f_rusty_line_input::Printer;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let mut config = Config::default();
    config.read_file();

    let mut client = Client::new(config);

    let mut reader = client.take_incoming_rx();
    let mut solver = CommandsSolver::default();

    let (input_tx, mut input_rx) = mpsc::channel::<String>(64);
    let printer_rx = spawn_rustyline_input(input_tx);
    let mut printer = printer_rx
        .await
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "printer not created"))?;

    print_cli(&mut printer, format!("Hello {}! Welcome chat CLI", client.get_name()).as_str(), ETextColor::White).await;
    print_cli(&mut printer, "Use /help to see all the commands", ETextColor::White).await;

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
                        if (val == ECommand::Quit){
                            disconnect(&mut printer, &mut client).await;
                        }
                    }
                    None => {
                        disconnect(&mut printer, &mut client).await;
                    }
                }
            }
        }
    }
    Ok(())
}
async fn disconnect(printer: &mut Printer, client: &mut Client) {
    print_cli(printer, "server/reader task closed channel", ETextColor::Red).await;
    client.disconnect().await;
}
