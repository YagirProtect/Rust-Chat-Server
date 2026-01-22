use crate::client_lib::classes::e_text_color::ETextColor;
use crate::client_lib::cli_utils::f_rusty_line_input::Printer;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::io::{stdout, Write};
use std::time::Duration;
use tokio::time::sleep;


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
    let mut out = stdout();
    let _ = execute!(out, Clear(ClearType::All), MoveTo(0, 0));
    let _ = out.flush();
}