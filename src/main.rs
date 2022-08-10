
use std::net::TcpStream;

use crate::mq::QueueControl;

mod mq;
mod server;
mod constants;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use threadpool::ThreadPool;
use std::sync::Arc;
use std::sync::Mutex;


struct Quey{
    tcp_server: Arc<server::Server>,
    queue_control: QueueControl, 
    start_time: Duration, 
    pool: ThreadPool, 
}

impl Quey{
    fn new()->Quey{
        let pool = ThreadPool::new(10);
        Quey { 
            tcp_server: Arc::new(server::Server::new(5896,pool)), 
            queue_control: QueueControl::new(), 
            start_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(), 
            pool: pool }
    }
    fn init(&mut self){
        // self.tcp_server = server::Server::new(5896);
        // self.queue_control = QueueControl::new();
        // self.start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        self.build_routes();
    }
    
    pub fn receive_publish(&mut self, stream: TcpStream, body:Vec<u8> ){    
        println!("F1 called {}",String::from_utf8_lossy(body.as_slice()));
    }
    pub fn receive_consumer_register(&mut self, stream: TcpStream, body:Vec<u8>){
        println!("F2 called {}",String::from_utf8_lossy(body.as_slice()));
    }
    fn build_routes(&mut self){
        
        let func = Box::new(self::Quey::receive_consumer_register);
        self.tcp_server.add_route(constants::MSG_PUBLISH_REQ, func);
        self.tcp_server.add_route(constants::CONSUMER_REGISTER_REQ, self::receive_consumer_register);

    }

}

fn main() {
    let now = SystemTime::now();
    println!("now {:?}",now.duration_since(UNIX_EPOCH)
    .expect("Time went backwards"));
    let mut quey = Quey::new();
    quey.build_routes();
    //router.route_function[&2001]("bosta".to_string());
    
    // println!("Hello, world! {:?}",mq::publish("bosta"));
    // let mut s: server::Server = server::Server::new(5896);
    // s.add_route(constants::MSG_PUBLISH_REQ, receive_publish);
    // s.add_route(constants::CONSUMER_REGISTER_REQ, receive_consumer_register);
    // let mut qc = QueueControl::new();
    // let mut nq1 = qc.new_queue(String::from("general"));
    // let mut id = nq1.publish("00000".to_string());
    // println!("total msg {}",nq1.total_messages());
    // let mut id = nq1.publish("00000".to_string());
    // let mut nq2 = qc.get("general".to_string());
    // let mut id = nq2.publish("00000".to_string());
    // println!("second total msg {}",nq2.total_messages());
    // println!("total msg {}",nq1.total_messages());
    // s.listen()
}

// fn receive_publish(stream: TcpStream, body:Vec<u8> ){
    
//     println!("F1 called {}",String::from_utf8_lossy(body.as_slice()));
// }
// fn receive_consumer_register(stream: TcpStream, body:Vec<u8>){
//     println!("F2 called {}",String::from_utf8_lossy(body.as_slice()));
// }