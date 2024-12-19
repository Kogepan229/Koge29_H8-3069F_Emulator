#[cfg(not(test))]
use anyhow::Result;
#[cfg(not(test))]
use std::{
    io::{BufRead, BufReader, BufWriter, ErrorKind, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

#[cfg(not(test))]
pub struct Socket {
    message_tx: Sender<String>,
    message_rx: Receiver<String>,
}

#[cfg(not(test))]
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
            writer.get_mut().shutdown(std::net::Shutdown::Both).unwrap();
        });
    }

    fn start_receive_worker(mut reader: BufReader<TcpStream>, message_tx: Sender<String>) {
        thread::spawn(move || {
            let mut message = String::new();
            loop {
                match reader.read_line(&mut message) {
                    Ok(n) => {
                        if n == 0 {
                            break;
                        }
                    }
                    Err(e) => {
                        if e.kind() != ErrorKind::ConnectionAborted {
                            eprintln!("{}", e);
                        }
                        break;
                    }
                }
                message_tx.send(message.replace('\n', "")).unwrap();
                message.clear();
            }
        });
    }

    pub fn pop_messages(&self) -> Result<Vec<String>> {
        Ok(self.message_rx.try_iter().collect())
    }

    pub fn send_message(&self, message: &String) -> Result<()> {
        self.message_tx.send(message.clone())?;
        Ok(())
    }
}
