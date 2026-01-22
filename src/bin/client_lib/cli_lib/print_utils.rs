use std::time::Duration;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use crate::client_lib::cli_lib::rusty_line_input::Printer;
pub enum ETextColor{
    White,
    Yellow,
    Red,
    Green
}


pub async fn print_cli(printer: &mut Printer, msg: &str, color: ETextColor) {
    sleep(Duration::from_millis(1)).await;

    let message = match color{
        ETextColor::White => msg.to_string(),
        ETextColor::Yellow => format!("\x1b[33m{} {}\x1b[0m", "", msg),
        ETextColor::Red => format!("\x1b[31m{} {}\x1b[0m", "[Error]", msg),
        ETextColor::Green => format!("\x1b[32m{} {}\x1b[0m", "[Info]", msg),

    };


    printer.print(message);
}

pub fn clear_console() {
    print!("\x1b[2J\x1b[H");
    let _ = io::stdout().flush();
}