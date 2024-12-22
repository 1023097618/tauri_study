# enum枚举

通过`enum`关键字，可以定义一个最基本的枚举变量

``` rust
fn main() {
    let a=IpAddress::V4;
    
}
enum IpAddress{
    V4,
    V6
}
```

# 给enum枚举附加上数据
使用以下的关键字来给enum枚举附加上一些数据

``` rust
fn main() {
    let a=IpAddress::V4(127,0,0,1);
}
enum IpAddress{
    V4(u8,u8,u8,u8),
    V6Q
}
```

# Option类型
`Option`是个`枚举`，一个是`Some`，表示有值，一个是`None`，表示没有值

这么做的好处就是强制提醒你将Some转换为其中的值，否则你就无法使用该值，以下是示例，这个示例中的代码是错的，因为Some类型必须转化成相应的数字雷系才能和c相加。
``` rust
fn main() {
    let a=Some(3);
    let c=3;
    let d=a+c;
}
```
# match匹配
通过`match`匹配一个enum对象可以实现对不同的enum值执行不同的代码块，有点类似于其他语言中的`switch`

但是`match`匹配必须枚举enum中所有可能的情况，否则会编译不通过
``` rust
#[derive(Debug)]
enum State{
    China,
    US
}


enum Coin{
    Cent,
    Yuan,
    Hundred,
    Origin(State)
}


fn main() {
    let coin=Coin::Origin(State::China);
    let value=match_coin(coin);
    println!("value is {}",value);
}
fn match_coin(coin:Coin)->u8{
    match coin {
        Coin::Cent=>1,
        Coin::Yuan=>2,
        Coin::Hundred=>3,
        Coin::Origin(state)=>{
            println!("state origin from {:?}",state);
            25
        }
    }
}
```

当然`match`也可以用于匹配其他数据类型的变量，如果要表示剩余的所有情况，可以使用`_`(当然也可以是别的)来作为通配符
``` rust
fn main() {
    let u:i8=3;
    match u {
        0=>println!("0"),
        3=>println!("3"),
        _=>()
    };
}

```

# if let语句
`if let`是`match`的语法糖，可以只针对某种特定的模式匹配代码

``` rust
fn main() {
    let u: Option<usize>=Some(3);
    if let Some(3)=u{
        println!("three");
    };
}
```