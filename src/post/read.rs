use std::io;

pub fn read_u32() -> u32{
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let action: u32 = guess.trim().parse().expect("Please type a number!");

    action
}
pub fn read_string() -> String{
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let action: String  = guess.trim().parse().expect("please type string");

    action
}
