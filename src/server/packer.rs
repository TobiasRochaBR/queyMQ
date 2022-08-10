
pub fn unpack(buffer: Vec<u8>)->super::tcp_message::Message{
    let mut header_size = &buffer[0..4];
    let mut header_id = &buffer[4..8];
    let id: u32 = header_id.read_u32::<LittleEndian>().unwrap();
    let size: u32 = header_size.read_u32::<LittleEndian>().unwrap();  
    let body = buffer[8..].to_vec();
    tcp_message::Message::new(id, size, body)
}