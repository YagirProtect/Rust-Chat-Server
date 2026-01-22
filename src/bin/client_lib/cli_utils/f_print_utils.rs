use crate::client_lib::classes::e_text_color::ETextColor;
use crate::client_lib::cli_utils::f_rusty_line_input::Printer;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::io::{stdout, Write};
use std::time::Duration;
use crossterm::style::Stylize;
use tokio::time::sleep;


pub async fn print_cli(printer: &mut Printer, msg: &str, color: ETextColor) {
    sleep(Duration::from_millis(1)).await;

    let message = match color{
        ETextColor::White => msg.to_string(),
        ETextColor::Yellow => msg.yellow().to_string(),
        ETextColor::Red => msg.red().to_string(),
        ETextColor::Green => msg.green().to_string(),

    };


    printer.print(message);
}

pub fn clear_console() {
    let mut out = stdout();
    let _ = execute!(out, Clear(ClearType::All), MoveTo(0, 0));
    let _ = out.flush();
}