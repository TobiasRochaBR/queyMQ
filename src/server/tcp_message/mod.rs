


#[derive(Clone)]
pub struct Message{
    id: u32,
    size: u32,
    body: Vec<u8>,
}

impl Message {
    pub fn new(id: u32, size:u32,body:Vec<u8>)->Message{
        let m = Message{
            id,
            size,
            body,
        };
        m
    }
    pub fn get_id(self)->u32{self.id.clone()}
    pub fn get_body(self)->Vec<u8>{self.body.clone()}
    pub fn get_size(self)->u32{self.size.clone()}
}


