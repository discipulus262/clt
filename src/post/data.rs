#[derive(Debug)]
pub enum Status {
    Read, 
    Unread, 
}
#[derive(Debug)]
pub struct Post {
    pub status: Status,
    pub content: String,
    pub author: String,
}
