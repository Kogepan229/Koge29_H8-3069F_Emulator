use anyhow::Result;
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

pub struct Socket {
    message_tx: Sender<String>,
    message_rx: Receiver<String>,
}

impl Socket {
    pub fn connect(addr: &String) -> Result<Self> {
        let listener = TcpListener::bind(addr)?;
        let (stream, _) = listener.accept()?;
        let reader = BufReader::new(stream.try_clone()?);
        let writer = BufWriter::new(stream);

        let (send_tx, send_rx) = mpsc::channel();
        let (receive_tx, receive_rx) = mpsc::channel();

        Self::start_send_worker(writer, send_rx);
        Self::start_receive_worker(reader, receive_tx);
        Ok(Self {
            message_tx: send_tx,
            message_rx: receive_rx,
        })
    }

    pub fn clonse_message_tx(&self) -> Sender<String> {
        self.message_tx.clone()
    }

    fn start_send_worker(mut writer: BufWriter<TcpStream>, message_rx: Receiver<String>) {
        thread::spawn(move || {
            while let Ok(message) = message_rx.recv() {
                let _msg = message.replace('\\', "\\\\").replace('\n', "\\n") + "\n";
                writer.write_all(_msg.as_bytes()).unwrap();
                writer.flush().unwrap();
            }
        });
    }

    fn start_receive_worker(mut reader: BufReader<TcpStream>, message_tx: Sender<String>) {
        thread::spawn(move || {
            let mut message = String::new();
            loop {
                if reader.read_line(&mut message).unwrap() == 0 {
                    break;
                }
                println!("recv: {}", message.replace('\n', ""));
                message_tx.send(message.replace('\n', "")).unwrap();
                message.clear();
            }
        });
        println!("owatta");
    }

    pub fn pop_messages(&mut self) -> Result<Vec<String>> {
        // println!("before pop");
        let g = self.message_rx.try_iter().collect();
        // println!("after pop");
        Ok(g)
    }

    pub fn send_message(&self, message: &String) -> Result<()> {
        // let _msg = message.replace('\\', "\\\\").replace('\n', "\\n") + "\n";
        // println!("write_all");
        // self.socket_writer.write_all(_msg.as_bytes())?;
        // self.socket_writer.flush()?;
        self.message_tx.send(message.clone())?;
        Ok(())
    }

    // pub fn send_one_sec_message(&mut self) -> Result<()> {
    //     self.send_message(&"1sec".to_string())?;
    //     Ok(())
    // }

    // pub fn send_addr_value_u8(&mut self, addr: u32, value: u8) -> Result<()> {
    //     let str = format!("u8:{:x}:{:x}", addr, value);
    //     self.send_message(&str)?;
    //     Ok(())
    // }

    // pub fn send_addr_value_u16(&mut self, addr: u32, value: u16) -> Result<()> {
    //     let str = format!("u16:{:x}:{:x}", addr, value);
    //     self.send_message(&str)?;
    //     Ok(())
    // }

    // pub fn send_addr_value_u32(&mut self, addr: u32, value: u32) -> Result<()> {
    //     let str = format!("u32:{:x}:{:x}", addr, value);
    //     self.send_message(&str)?;
    //     Ok(())
    // }
}
