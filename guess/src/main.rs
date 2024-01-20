use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("秘密数字是：{}", secret_number);
    println!("猜测一个数");
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");

    let guess: u32 = guess.trim().parse().expect("please type a number");
    println!("你猜测的数字是：{}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("small"),
        Ordering::Greater => println!("big"),
        Ordering::Equal => println!("you win"),
    };
}
