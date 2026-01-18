use futures_util::SinkExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use crate::shared_lib::c_commands_solver::{CommandsSolver, ECommand, ECommandType};

pub struct Client {
    name: String,
    last_address: Option<String>,
    writer: Option<OwnedWriteHalf>,
    reader: Option<JoinHandle<()>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            name: "user".to_string(),
            writer: None,
            reader: None,
            last_address: None,
        }
    }


    pub fn is_connected(&self) -> bool {
        self.writer.is_some()
    }
    pub fn get_last_address(&self) -> Option<String> {
        self.last_address.clone()
    }

    pub async fn connect(&mut self, addr: &str) -> std::io::Result<()> {
        let stream = TcpStream::connect(addr).await?;
        let (rd, wr) = stream.into_split();

        let handle = tokio::spawn(async move {
            read_from_server(rd).await;
        });


        self.writer = Some(wr);
        self.reader = Some(handle);

        println!("Connected to {addr}");
        
        let command = CommandsSolver::create_command(ECommand::CreateUser, self.name.clone(), ECommandType::ToServer);
        
        self.send_message(command).await;
        
        Ok(())
    }

    pub async fn disconnect(&mut self) {
        self.reader.take();
        if let Some(mut w) = self.writer.take() {
            let _ = w.shutdown().await;
        }


        self.reader = None;
        self.writer = None;
    }

    pub fn change_name(&mut self, name: &str) {
        self.name = name.to_string();
    }


    pub async fn send_message(&mut self, message: String) {
        if (self.is_connected()) {
            let Some(writer) = self.writer.as_mut() else {
                println!("No writer (not connected?)");
                return;
            };

            if writer.write_all(message.as_bytes()).await.is_err() {
                println!("Failed to send message");
                return;
            }

            if writer.write_all(b"\n").await.is_err() {
                println!("Failed to send newline");
            }
        }
    }
}

async fn read_from_server(mut rd: OwnedReadHalf) {
    let mut buf = [0u8; 4096];

    loop {
        let n = match rd.read(&mut buf).await {
            Ok(0) => {
                println!("\n[server closed connection]");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                println!("\n[read error: {e}]");
                break;
            }
        };

        print!("{}", String::from_utf8_lossy(&buf[..n]));
    }
}