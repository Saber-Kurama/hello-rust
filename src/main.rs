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

    // struct User {
    //     username: &str,
    //     email: &str,

    // }

    // let width1 = 39;
    // let height1 = 50;

    // println!("The area of the rectangle is {} square pixels.",
    // area(width1, height1))
    // let rect1 = (30, 50);

    // println!("The area of the rectangle is {} square pixels.",
    // area(rect1))

    // let rect1 = Rectangle{ width: 30, height: 50};

    // println!("The area of the rectangle is {:#?} square pixels.",
    // rect1.area());
    // dbg!(&rect1);

    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    struct IpAddrKind {
        kind: IpAddr,
        address: String,
    }
    let home1 = IpAddrKind {
        kind: IpAddr::V6(String::from("::1")),
        address: String::from("127.0.0.1"),
    };
    println!("enum的枚举值是{:?}", home);
    // dbg!(IpAddr);

    let v = vec![1, 2, 3];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

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
