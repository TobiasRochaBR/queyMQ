mod message;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Queue{    
    storage: Arc<Mutex<HashMap<String, message::Message>>>
}

impl Queue{
    pub fn new()->Queue{
        Queue { storage: Arc::new(Mutex::new(HashMap::new())) }
    }
    pub fn publish(&mut self,data: String)->String{
        let mut msg = message::new_message(data);
        self.storage.lock().unwrap().insert(msg.id.clone(), msg.clone());
        println!("msg published {}",msg.id.clone());
        msg.id.clone()
    }
    pub fn total_messages(&mut self)->usize{
        self.storage.lock().unwrap().len()
    }
}
pub struct QueueControl{
    storage: Arc<Mutex<HashMap<String, Queue>>>
}

impl QueueControl{
    pub fn new()->QueueControl{
        QueueControl { storage: Arc::new(Mutex::new(HashMap::new())) }        
    }

    pub fn new_queue(&mut self,name:String)->Queue{
        println!("initializing queue {}",name.clone());
        self.storage.lock().unwrap().insert(name.clone(), Queue::new());
        self.storage.lock().unwrap()[&name.clone()].clone()
    }
    pub fn get(&mut self,name:String)->Queue{
        self.storage.lock().unwrap()[&name.clone()].clone()
    }
}

pub fn publish(data: &str)->message::Message{
    message::new_message(String::from(data))
}