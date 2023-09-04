mod bus;
mod cpu;
mod elf;
mod memory;
mod setting;

use clap::Parser;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::cpu::Cpu;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// path of the elf file to execute
    #[arg(short, long)]
    elf: String,

    /// Print executed opcode
    #[arg(long)]
    debug_instruction: Option<bool>,

    #[arg(short, long)]
    disable_socket: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Some(v) = args.debug_instruction {
        *setting::ENABLE_PRINT_OPCODE.write().unwrap() = v;
    }

    let mut cpu = Cpu::new();

    if !args.disable_socket {
        let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
        let (stream, _) = listener.accept().await.unwrap();
        let (reader, _writer) = stream.into_split();
        let writer = Arc::new(Mutex::new(_writer));
        cpu.emu_share_values.lock().await.socket_writer = Some(writer.clone());
    }


    /* // test code
    let cpu_emu_share = cpu.emu_share_values.clone();
    tokio::spawn(async move {
        loop {
            println!("{}", cpu_emu_share.lock().await.pc)
        }
    });
     */

    elf::load(args.elf, &mut cpu).await;
    // print_memory(&cpu.bus.memory);

    cpu.run().await.unwrap();
}
