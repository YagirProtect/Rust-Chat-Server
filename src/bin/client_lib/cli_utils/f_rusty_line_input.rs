use rustyline::{DefaultEditor, ExternalPrinter, error::ReadlineError};
use rustyline::config::Configurer;
use tokio::sync::{mpsc, oneshot};

pub type Printer = Box<dyn ExternalPrinter + Send>;

pub fn spawn_rustyline_input(input_tx: mpsc::Sender<String>) -> oneshot::Receiver<Printer> {
    let (printer_tx, printer_rx) = oneshot::channel::<Printer>();

    tokio::task::spawn_blocking(move || {
        let mut rl = DefaultEditor::new().expect("rustyline init failed");

        rl.set_auto_add_history(true);

        let printer = rl.create_external_printer().expect("external printer failed");
        let _ = printer_tx.send(Box::new(printer));

        loop {
            match rl.readline(">> ") {
                Ok(line) => {
                    let line = line.trim().to_string();
                    if line.is_empty() { continue; }

                    if input_tx.blocking_send(line).is_err() {
                        break;
                    }
                }
                Err(ReadlineError::Interrupted) => continue, // Ctrl+C
                Err(ReadlineError::Eof) => break,            // Ctrl+D
                Err(e) => {
                    eprintln!("readline error: {e}");
                    break;
                }
            }
        }
    });

    printer_rx
}
