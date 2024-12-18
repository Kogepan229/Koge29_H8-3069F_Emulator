use anyhow::Result;
use std::sync::{Mutex, OnceLock};
use tokio::{
    io, join,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    sync::{
        mpsc::{self, Receiver, Sender},
        RwLock,
    },
    task::JoinHandle,
};

use crate::setting;

static READ_BUF: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
static MESSAGE_SENDER: OnceLock<RwLock<Option<Sender<String>>>> = OnceLock::new();
static JOIN_HANDLES: OnceLock<Mutex<Vec<JoinHandle<()>>>> = OnceLock::new();

pub async fn listen(addr: String) -> Result<()> {
    let listener = TcpListener::bind(addr).await.unwrap();

    let (stream, _) = listener.accept().await.unwrap();
    let (socket_reader, socket_writer) = stream.into_split();

    READ_BUF.set(Mutex::new(Vec::<String>::new())).unwrap();
    start_receive_worker(socket_reader);

    JOIN_HANDLES.set(Mutex::new(Vec::new())).unwrap();
    let (tx, rx) = mpsc::channel(32);
    MESSAGE_SENDER.set(RwLock::new(Some(tx))).unwrap();
    start_send_workder(rx, socket_writer);
    Ok(())
}

fn start_receive_worker(socket_reader: OwnedReadHalf) {
    tokio::spawn(async move {
        let mut message: Vec<u8> = Vec::new();
        loop {
            let mut received = vec![0; 1024];
            socket_reader.readable().await.unwrap();
            match socket_reader.try_read(&mut received) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    received.truncate(n);
                    match READ_BUF.get() {
                        Some(b) => {
                            for ch in received {
                                if ch == b'\n' {
                                    b.lock()
                                        .unwrap()
                                        .push(String::from_utf8(message.clone()).unwrap().trim().to_string());
                                    message.clear();
                                } else {
                                    message.push(ch);
                                }
                            }
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
    let job = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let _msg = message.replace('\\', "\\\\").replace('\n', "\\n") + "\n";
            let str_bytes = _msg.as_bytes();
            let mut written_bytes = 0;
            println!("{}", _msg);
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
    JOIN_HANDLES.get().unwrap().lock().unwrap().push(job);
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

pub fn filter_handles() {
    // let mut handles = JOIN_HANDLES.get().unwrap().lock().unwrap();
    // let new: Vec<JoinHandle<()>> = handles.iter().filter(|job| !job.is_finished());
    if let Some(handles_mutex) = JOIN_HANDLES.get() {
        let mut handles = handles_mutex.lock().unwrap();
        let mut new_handles = vec![];
        while let Some(job) = handles.pop() {
            if !job.is_finished() {
                new_handles.push(job);
            }
        }
        *handles = new_handles;
    }
}

pub async fn wait_sending() {
    if let Some(v) = MESSAGE_SENDER.get() {
        *v.write().await = None;
    }
    if let Some(handles_mutex) = JOIN_HANDLES.get() {
        let mut handles = handles_mutex.lock().unwrap();
        while let Some(job) = handles.pop() {
            job.await.unwrap();
        }
    }
}

pub fn send_message(str: &str) {
    if let Some(v) = MESSAGE_SENDER.get() {
        let _str = str.to_string();
        let job = tokio::spawn(async move {
            let sender = v.read().await;
            sender.as_ref().unwrap().send(_str).await.unwrap();
        });
        JOIN_HANDLES.get().unwrap().lock().unwrap().push(job);
    }
    if *setting::ENABLE_PRINT_MESSAGES.read().unwrap() {
        println!("msg: {}", str);
    }
}

pub fn send_one_sec_message() {
    send_message("1sec");
}

pub fn send_addr_value_u8(addr: u32, value: u8) {
    let str = format!("u8:{:x}:{:x}", addr, value);
    send_message(&str);
}

pub fn send_addr_value_u16(addr: u32, value: u16) {
    let str = format!("u16:{:x}:{:x}", addr, value);
    send_message(&str);
}

pub fn send_addr_value_u32(addr: u32, value: u32) {
    let str = format!("u32:{:x}:{:x}", addr, value);
    send_message(&str);
}
