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
    // let sum = 5.0 + 10.0;
    // println!("加法---: {}", sum);

    // let mut s = String::from("hello");

    // s.push_str(", world!"); // push_str() 在字符串后追加字面值

    // println!("{}", s); // 将打印 `hello, world!`

    // 移动
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);

    // 克隆
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);

    // 只在栈上的数据：拷贝
    // let x = 5;
    // let y = x;
    // println!("x = {}, y = {}", x, y);

    // let s = String::from("hello");
    // takes_ownership(s);
    // // println!("{}", s); s 已经不再有效
    // let x = 5;
    // makes_copy(x);
    // println!("{}", x)

    // let s1 = String::from("hello");

    // let len = calculate_length(&s1);

    // println!("The length of '{}' is {}.", s1, len);

    // let mut s = String::from("hello");

    // change(&mut s);

    // 可变引用的限制
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // // println!("{}, {}", r1, r2);
    // // println!("{}", r1);
    // // println!("{}", r2);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}
fn makes_copy(some_interger: i32) {
    println!("{}", some_interger)
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() 返回字符串的长度

//     (s, length)
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}
// 不允许修改引用的值
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// 可变引用
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// // 悬垂引用
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
