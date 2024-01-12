mod read;
mod data;

pub fn summarize(post: data::Post){
    println!("new post from {} (press 1 to read more)", post.author);
    let choice = read::read_u32();
    if choice == 1 {
        println!("{} wrote {}", post.author, post.content);
        //add to database with status read for name
    }
}
pub fn get_uname() -> String{
    println!("what do you want people to call you");
    let uname = read::read_string();
    uname
}
pub fn post(uname: String) -> data::Post {
    println!("type your post");
    let msg = read::read_string();
    data::Post{
        status: data::Status::Unread,
        content: msg,
        author: uname, 
    }
}
pub fn read() -> u32{
    let result = read::read_u32();
    result
}
pub fn help(){
    println!("helP");
}
