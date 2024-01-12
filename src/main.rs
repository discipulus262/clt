mod post; 
fn init_post(name: String){
    let post = post::post(name);
    post::summarize(post);
}
fn main() {
    let name = post::get_uname();
    println!("enter 1 to read, 2 to post 3 for help");
    let action = post::read();
    match action {
        1 => println!("not work yet"),
        2 => init_post(name),
        3 => post::help(),
        4 => hello(name),
        _  => println!("not valid argument"),
    }
}
fn hello(hello: String){
    println!("hello {hello}");
}
