//Kinesys Rust Tcp chat Server

use std::io::{Errorkind, Read, Write}
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

//const 선언
const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
	
	let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
	server.set_nonblocking(true).expect("failed to initialize non-blocking");
    
}
