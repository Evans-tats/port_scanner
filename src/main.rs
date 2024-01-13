use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

use bpaf::Bpaf;

const MAX: u32 = 65536;
const IPFALLBACK :IpAddr = IpAddr::V4((Ipv4Addr::new(127, 0, 0, 1)));
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments {
    #[bpaf(long, short, fallback(IPFALLBACK))]

    pub address: IpAddr,
    #[bpaf(
        long("start"),
        short('s'),
        fallback(1),
        guard(start_port_guard, "must be greater than 0")

    )]
    pub start_port: u32,

    #[bpaf(
        long("end"),
        short('e'),
        fallback(MAX),
        guard(stop_port_guard, "Must be less than or equal to 65535")

    )]
    pub end_port: u32,

}

fn start_port_guard(input: &u32) -> bool {
    *input > 0
}
fn stop_port_guard(input : &u32) -> bool {
    *input <= MAX
}

async fn scan(tx: Sender<u32>, port: u32, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}",addr,port)).await {
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        Err(_) => {}
    }

}   


#[tokio::main]
async fn main() {
    let opts: Arguments = arguments().run();
    let(tx, rx) = channel();
    for i in opts.start_port..opts.end_port {
        let tx = tx.clone();
        task::spawn(async move {
            scan(tx,i,opts.address).await;
        });
    }
    let mut out = vec![];
    drop(tx);
    for p in rx {
         out.push(p);
    }
    println!("");
    out.sort();
    for i in out {
        println!("{} is open",i);
    }
        
    

}   
