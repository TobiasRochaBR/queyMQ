
use uuid::Uuid;
extern crate base64;

#[derive(Debug,Clone)]
pub struct Message{
    pub id: String,
    pub body: String
}

pub fn new_message(body: String)->Message{
    let uid = Uuid::new_v4();
    let idb64 = base64::encode(uid);
    let m = Message{
        id:  idb64,
        body:  body,
    };
    m
}