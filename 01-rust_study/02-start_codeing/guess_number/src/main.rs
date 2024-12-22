use std::{cmp::Ordering, io};
use rand::Rng;
fn main() {
    let secret_number=rand::thread_rng().gen_range(0, 100);
    println!("神秘数字是{}",secret_number);
    println!("猜数字");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取信息");
        let guess:u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue
        }; 
        println!("你猜测的数字是{}",guess);
        match guess.cmp(&secret_number){
            Ordering::Less => println!("猜小了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            },
            Ordering::Greater => println!("猜大了"),
        }
    }
}
