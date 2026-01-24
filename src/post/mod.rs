mod read;
mod tomlreader;
pub fn summarize(msg: String) {
    println!("new task created: {}", msg);
}

pub fn post() -> String {
    println!("type your post");
    let msg = read::read_string();
    msg
}
pub fn read() -> u32 {
    let result = read::read_u32();
    result
}
pub fn help() {
    println!("CLT, a task manager for the terminal");
}
pub fn write_data(taskname: String) {
    tomlreader::write(taskname)
}
