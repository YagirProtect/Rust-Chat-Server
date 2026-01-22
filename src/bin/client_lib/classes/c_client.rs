use crate::shared_lib::c_command::Packet;
use crate::shared_lib::c_commands_solver::{CommandsSolver, ECommand};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use crate::shared_lib::c_config::Config;

pub struct Client {
    server_id: u32,
    is_in_room: bool,
    name: String,
    last_address: Option<String>,
    default_address: String,
    config: Config,

    in_channel_rx: Option<mpsc::Receiver<Packet>>,
    in_channel_tx: mpsc::Sender<Packet>,

    out_channel_tx: mpsc::Sender<String>,
    out_channel_rx: Option<mpsc::Receiver<String>>,

    writer: Option<JoinHandle<()>>,
    reader: Option<JoinHandle<()>>,
}



impl Client{
    pub fn new(config: Config) -> Self {
        let (out_tx, out_rx) = mpsc::channel::<String>(256);
        let (in_tx, in_rx) = mpsc::channel::<Packet>(256);

        Self {
            server_id: 0,
            is_in_room: false,
            name: config.user_name(),
            writer: None,
            reader: None,
            last_address: None,

            in_channel_rx: Some(in_rx),
            in_channel_tx: in_tx,
            out_channel_tx: out_tx,
            out_channel_rx: Some(out_rx),
            default_address: config.get_address(),
            config: config,
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn is_in_room(&self) -> bool{
        self.is_in_room
    }
    pub fn set_in_room(&mut self, state: bool){
        self.is_in_room = state
    }
    pub fn set_id(&mut self, id: u32) {
        self.server_id = id;
    }
    pub fn is_connected(&self) -> bool {
        self.writer.is_some()
    }

    pub async fn connect(&mut self, addr: &str) -> std::io::Result<()> {

        let stream = TcpStream::connect(addr).await?;
        let (rd, wr) = stream.into_split();
        let (out_tx, out_rx) = mpsc::channel::<String>(256);


        self.last_address = Some(addr.to_string());
        self.out_channel_tx = out_tx;
        self.out_channel_rx = Some(out_rx);



        let out_rx = self.out_channel_rx
            .take()
            .expect("out_channel_rx already taken (already connected?)");

        let in_tx = self.in_channel_tx.clone();

        let writer_task = tokio::spawn(async move {
            writer_loop(wr, out_rx).await;
        });

        let reader_task = tokio::spawn(async move {
            reader_loop(rd, in_tx).await;
        });

        self.writer = Some(writer_task);
        self.reader = Some(reader_task);
        
        let packet = CommandsSolver::create_command(ECommand::CreateUser, [self.name.clone()]);
        self.send_message(packet).await;
        
        Ok(())
    }

    pub async fn disconnect(&mut self) {
        self.reader.take();
        if let Some(mut w) = self.writer.take() {
            w.abort();
        }
        if let Some(mut r) = self.reader.take() {
            r.abort();
        }

        println!("Disconnected from {:?}", self.last_address.clone().unwrap().as_str());
        self.reader = None;
        self.writer = None;
    }

    pub fn change_name(&mut self, name: &str) {
        self.config.set_user_name(name.to_string());
        self.name = name.to_string();

        self.config.write_file();
    }

    pub fn take_incoming_rx(&mut self) -> mpsc::Receiver<Packet> {
        self.in_channel_rx.take().expect("incoming rx already taken")
    }

    pub async fn send_message(&mut self, packet: Packet) {
        if (self.is_connected()) {
            self.out_channel_tx.send(format!("{}\n", packet.to_string())).await.expect("sender empty!!");
        }
    }

    pub fn get_default_address(&self) -> String {
        self.default_address.clone()
    }
}

pub async fn writer_loop(mut wr: OwnedWriteHalf, mut out_rx: mpsc::Receiver<String>) {
    while let Some(mut msg) = out_rx.recv().await {
        if !msg.ends_with('\n') {
            msg.push('\n');
        }
        if wr.write_all(msg.as_bytes()).await.is_err() {
            break;
        }
    }
}

async fn reader_loop(rd: OwnedReadHalf, in_tx: mpsc::Sender<Packet>) {
    let mut lines = BufReader::new(rd).lines();
    while let Ok(Some(line)) = lines.next_line().await {
        let ev = Packet::load(line.as_str());
        if in_tx.send(ev).await.is_err() {
            return;
        }
    }

    let _ = in_tx.send(Packet::new(ECommand::None, Vec::<String>::new())).await;
}