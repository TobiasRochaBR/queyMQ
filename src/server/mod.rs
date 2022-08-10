
use std::io::prelude::*;
use std::net::{TcpListener,TcpStream};
use std::vec;
use byteorder::{LittleEndian, ReadBytesExt};
pub mod tcp_message;
mod router;
use std::process;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use threadpool::ThreadPool;


pub struct ServerInner{
    pid: u32,
    router: router::RouterHandler,
    port: u16,    
}


pub struct Server{
    inner: Arc<Mutex<ServerInner>>,
    pool: ThreadPool
}

impl Server {
    pub fn new(port: u16, pool:ThreadPool)->Server{
        //My {inner: Arc::new(MyInner {s: Mutex::new(s)})}
        let s: Server = Server{ 
            pool: pool,
            inner: Arc::new(Mutex::new(
                ServerInner { 
                    pid: process::id(), 
                    router: router::RouterHandler::new(), 
                    port: port, }
            ))
            
        };
        s
    }
    pub fn add_route(&mut self,id:u32,func: router::Callback){
        let local_self = self.inner.clone();
        local_self.lock().unwrap().router.add_route(id, func);
    }
    pub fn listen(&mut self){
        let pool = self.pool;
        //println!("Server starting! {:?}",self);
        let listener = TcpListener::bind("127.0.0.1:5896").unwrap();
        loop {
            let mut local_self = self.inner.clone();
            let (stream, socket_address) = listener.accept().unwrap();
            println!("Got new client whose address is {}", socket_address);
            println!("Stream: {:?}", stream);
            pool.execute(move || local_self.lock().unwrap().handle_connection(stream));
        }
        // for stream in listener.incoming() {
        //     let stream = stream.unwrap();
        //     self.handle_connection(stream);
        // }
    }
    

    
}


impl ServerInner {
    fn handle_connection(&mut self,mut stream: TcpStream) {
        
        let (mut buffer, _request_len) = read_stream(&mut stream);
        // unpacking
        let mut header_size = &buffer[0..4];
        let mut header_id = &buffer[4..8];
        let id: u32 = header_id.read_u32::<LittleEndian>().unwrap();
        let size: u32 = header_size.read_u32::<LittleEndian>().unwrap();  
        let body = buffer[8..].to_vec();
        println!("Request: id:{}, size:{}", id,size);
        // tcp message from unpack
        //let m = tcp_message::Message::new(id, size, body);
        self.router.call_route(stream,id,body);
        
    }
    pub fn add_route(&mut self,id:u32,func: router::Callback){
        self.router.add_route(id, func);
    }
}

/// Read the stream data and return stream data & its length.
fn read_stream(stream: &mut TcpStream) -> (Vec<u8>, usize) {
    let buffer_size = 512;
    let mut request_buffer = vec![];
    // let us loop & try to read the whole request data
    let mut request_len = 0usize;
    loop {
        let mut buffer = vec![0; buffer_size];
        // println!("Reading stream data");
        match stream.read(&mut buffer) {
            Ok(n) => {
                // Added these lines for verification of reading requests correctly
                // println!("Number of bytes read from stream: {}", n);
                // println!(
                //     "Buffer data as of now: {}",
                //     String::from_utf8_lossy(&buffer[..])
                // );
                if n == 0 {
                    // Added these lines for verification of reading requests correctly
                    // println!("No bytes read");
                    break;
                } else {
                    request_len += n;

                    // we need not read more data in case we have read less data than buffer size
                    if n < buffer_size {
                        // let us only append the data how much we have read rather than complete existing buffer data
                        // as n is less than buffer size
                        request_buffer.append(&mut buffer[..n].to_vec()); // convert slice into vec
                                                                          // Added these lines for verification of reading requests correctly
                                                                          // println!("No Need to read more data");
                        break;
                    } else {
                        // append complete buffer vec data into request_buffer vec as n == buffer_size
                        request_buffer.append(&mut buffer);
                    }
                }
            }
            Err(e) => {
                println!("Error in reading stream data: {:?}", e);
                break;
            }
        }
        //println!("Stream read loop code ends here");
    }

    (request_buffer, request_len)
}
