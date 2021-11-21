// use ferris_says::say;
// use std::io::{stdout, BufWriter};

// fn main() {
//     let stdout = stdout();
//     let message = String::from("Hello fellow Rustaceans!");
//     let width = message.chars().count();
//     let mut writer = BufWriter::new(stdout.lock());
//     say(message.as_bytes(), width, &mut writer).unwrap();
// }

// use std::io;

fn main() {
    // println!("Guess the number!");

    // println!("Please input your guess.");

    // let mut guess = String::new();

    // io::stdin().read_line(&mut guess)
    //     .expect("Failed to read line");

    // println!("You guessed: {}", guess);

    // 加法
    let sum = 5.0 + 10.0;
    println!("加法---: {}", sum);
}