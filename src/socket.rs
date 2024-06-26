use anyhow::Result;
use std::sync::{Mutex, OnceLock};
use tokio::{
    io,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    sync::mpsc::{self, Receiver, Sender},
};

static READ_BUF: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
static MESSAGE_SENDER: OnceLock<Sender<String>> = OnceLock::new();

pub async fn listen(addr: String) -> Result<()> {
    let listener = TcpListener::bind(addr).await.unwrap();
    let (stream, _) = listener.accept().await.unwrap();
    let (socket_reader, socket_writer) = stream.into_split();

    READ_BUF.set(Mutex::new(Vec::<String>::new())).unwrap();
    start_receive_worker(socket_reader);

    let (tx, rx) = mpsc::channel(32);
    MESSAGE_SENDER.set(tx).unwrap();
    start_send_workder(rx, socket_writer);
    Ok(())
}

fn start_receive_worker(socket_reader: OwnedReadHalf) {
    tokio::spawn(async move {
        loop {
            let mut msg = vec![0; 1024];
            socket_reader.readable().await.unwrap();
            match socket_reader.try_read(&mut msg) {
                Ok(n) => {
                    msg.truncate(n);
                    match READ_BUF.get() {
                        Some(b) => {
                            b.lock().unwrap().push(String::from_utf8(msg).unwrap());
                        }
                        None => return,
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    println!("{}", e.to_string());
                    return;
                }
            }
        }
    });
}

fn start_send_workder(mut rx: Receiver<String>, socket_writer: OwnedWriteHalf) {
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let _msg = message + "\n";
            let str_bytes = _msg.as_bytes();
            let mut written_bytes = 0;
            loop {
                socket_writer.writable().await.unwrap();
                match socket_writer.try_write(str_bytes) {
                    Ok(n) => {
                        written_bytes += n;
                    }
                    Err(_) => {}
                }
                if written_bytes == str_bytes.len() {
                    break;
                }
            }
        }
    });
}

pub fn get_received_msgs() -> Option<Vec<String>> {
    match READ_BUF.get() {
        Some(b) => {
            let mut v = b.lock().unwrap();
            let r = Some(v.clone());
            v.clear();
            return r;
        }
        None => return None,
    }
}

pub async fn send_message(str: &str) {
    if let Some(v) = MESSAGE_SENDER.get() {
        v.send(str.to_string()).await.unwrap();
    }
}

pub async fn send_one_sec_message() {
    send_message("1sec").await;
}

pub async fn send_addr_value_u8(addr: u32, value: u8) {
    let str = format!("u8:0x{:x}:0x{:x}", addr, value);
    send_message(&str).await;
}

pub async fn send_addr_value_u16(addr: u32, value: u16) {
    let str = format!("u16:0x{:x}:0x{:x}", addr, value);
    send_message(&str).await;
}

pub async fn send_addr_value_u32(addr: u32, value: u32) {
    let str = format!("u32:0x{:x}:0x{:x}", addr, value);
    send_message(&str).await;
}
