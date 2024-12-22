>在这个例子中，我们使用了`猜数字`的例子来进行rust的入门

我们使用了`random包`,可以在[https://crates.io/crates/random](https://crates.io/crates/random)中找到关于它的说明

我感觉`match`有点像其他语言的swich？

然后需要使用`use rand::Rng;`这个`trait`才可以使用`rand::thread_rng().gen_range`有点像其他语言的import？

使用`mut`来说明这是一个可变的变量

``` rust
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

```