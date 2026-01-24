mod post;
fn init_post() {
    let pst = post::post();
    post::write_data(pst)
}
fn read_lines() {
    println!("good to read linex");
}
fn main() {
    //instuructions
    println!("enter 1 to read, 2 to post, 3 for help, 4 to quit");
    loop {
        print!(">");
        let action = post::read();
        match action {
            1 => read_lines(),
            2 => init_post(),
            3 => post::help(),
            4 => break,
            _ => println!("not valid argument"),
        }
    }
}
