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

    // 借用的示例
    // 切片
    // let s = String::from("hello, rust");
    // println!("{}", &s[0..5]);

    // let vec1 = vec![1, 2, 3, 4, 5];
    // println!("{:?}", &vec1[0..2]);
    // // 切片作为参数
    // // 可变切片 数据源是可变的，切片是可变的
    // let mut vec2 = vec![1,2,3,4,5];
    // let vec_slice = &mut vec2[0..2];
    // vec_slice[0] = 13;
    // println!("{:?}", vec2)

    // 示例2 迭代器
    // 迭代器 IntoIter Iter IterMut
    
    // let vec1 = vec!["java", "rust", "javascript"];
    // // 所有权转移
    // // for str in vec1.into_iter() {
    // //     match str {
    // //         "rust" => println!("hello reust"),
    // //         _ => println!("{}", str)
    // //     }
    // // }

    // // 不可变借用
    // for str in vec1.iter() {
    //     match str {
    //         &"rust" => println!("hello reust"),
    //         _ => println!("{}", str)
    //     }
    // }

    // // 可变借用
    // let mut vec = vec!["java", "rust", "javascript"];
    // for str in vec.iter_mut() {
    //     match str {
    //         &mut "rust" => {
    //             *str = "saber";
    //             println!("{}", str);
    //         },
    //         _ => println!("{}", str)
    //     }
    // }

    // println!("{:?}", vec)

    // 生命周期
    let str1 = String::from("abcd");
    let result;
    {
        // let str2 = "xyz"; // 这个是可以的
        let str2 = String::from("abcd的");
        result = long_str(str1.as_str(), str2.as_str());
    }
    println!("long {}", result)

}

fn long_str(x: &str, y: &str) -> bool {
    true
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
