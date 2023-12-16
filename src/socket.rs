use anyhow::Result;
use std::sync::{Mutex, OnceLock};
use tokio::{
    io,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
};

static READER: OnceLock<OwnedReadHalf> = OnceLock::new();
static WRITER: OnceLock<OwnedWriteHalf> = OnceLock::new();
static READ_BUF: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

pub async fn listen(addr: String) -> Result<()> {
    let listener = TcpListener::bind(addr).await.unwrap();
    let (stream, _) = listener.accept().await.unwrap();
    let (reader, writer) = stream.into_split();
    READER.set(reader).unwrap();
    WRITER.set(writer).unwrap();
    READ_BUF.set(Mutex::new(Vec::<String>::new())).unwrap();
    receive();
    Ok(())
}

fn receive() {
    match READER.get() {
        Some(s) => {
            tokio::spawn(async move {
                loop {
                    let mut msg = vec![0; 1024];
                    s.readable().await.unwrap();
                    match s.try_read(&mut msg) {
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
        None => return,
    }
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

pub fn send_addr_value_u8(addr: u32, value: u8) {
    match WRITER.get() {
        Some(v) => {
            tokio::spawn(async move {
                let str = format!("u8:{}:{}\n", addr, value);
                let str_bytes = str.as_bytes();
                let mut written_bytes = 0;
                loop {
                    v.writable().await.unwrap();
                    match v.try_write(str_bytes) {
                        Ok(n) => {
                            written_bytes += n;
                        }
                        Err(_) => {}
                    }
                    if written_bytes == str_bytes.len() {
                        break;
                    }
                }
            });
        }
        None => return,
    }
}

pub fn send_addr_value_u16(addr: u32, value: u16) {
    match WRITER.get() {
        Some(v) => {
            tokio::spawn(async move {
                let str = format!("u16:{}:{}\n", addr, value);
                let str_bytes = str.as_bytes();
                let mut written_bytes = 0;
                loop {
                    v.writable().await.unwrap();
                    match v.try_write(str_bytes) {
                        Ok(n) => {
                            written_bytes += n;
                        }
                        Err(_) => {}
                    }
                    if written_bytes == str_bytes.len() {
                        break;
                    }
                }
            });
        }
        None => return,
    }
}

pub fn send_addr_value_u32(addr: u32, value: u32) {
    match WRITER.get() {
        Some(v) => {
            tokio::spawn(async move {
                let str = format!("u32:{}:{}\n", addr, value);
                let str_bytes = str.as_bytes();
                let mut written_bytes = 0;
                loop {
                    v.writable().await.unwrap();
                    match v.try_write(str_bytes) {
                        Ok(n) => {
                            written_bytes += n;
                        }
                        Err(_) => {}
                    }
                    if written_bytes == str_bytes.len() {
                        break;
                    }
                }
            });
        }
        None => return,
    }
}
