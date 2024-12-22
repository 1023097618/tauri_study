# Vector
## Vector的定义
使用以下代码定义一个`vector`
``` rust
let v:Vec<i32>=Vec::new();
```
或者使用以下方式进行宏定义
``` rust 
let v=vec![1,2,3];
```
## Vector的访问
1. 使用索引方式访问vector，这种方式如果超出索引会引起程序的`panic`，从而退出程序
``` rust
let v=vec![1,2,3];
let third:&i32=&v[3];
println!("第三个值是{}",third);
```
2. 使用`get`的方式访问vector，那么会返回一个`Option`对象，我们需要通过一个`match`匹配来对`Option`进行匹配
``` rust
let v=vec![1,2,3];
let result=v.get(3);
match result {
    Some(third)=>println!("结果是{}",third),
    None=>print!("there is no element")
}
```
## vector的更改和遍历
1. 使用`push`方法来添加数据
2. 使用for语句实现vector元素的遍历

``` rust
fn main() {
    let mut v=vec![1,2,3];
    v.push(4);
    for i in &mut v{
        *i=*i+50;
    }
}
```

# String
## 字符串的获取
使用`push_str`可以往字符串中附加值
``` rust
fn main() {
    let mut s="abc".to_string();
    s.push_str("foo bar");
    println!("{}",s);
}
```
还可以使用`+`运算符来拼接字符串
``` rust
let s=String::from("hello");
let s1=String::from("world");
let s2=s+&s1;
```
本质上上述代码调用了这个函数
``` rust
fn add(self,s:&str)->String{
...
}
```
通过`format!`来方便快速地拼接字符串
``` rust
fn main() {
    let s=String::from("hello");
    let s1=String::from("world");
    let s2=String::from("rust");
    let s3=format!("{}-{}-{}",s,s1,s2);
    println!("{}",s3)
}
```

## 字符串的读取
可以使用`.bytes()`的方式获取
``` rust
fn main() {
    let s=String::from("你好");
    for b in s.bytes(){
        println!("{}",b);
    }
}
```
也可以使用`.chars()`的方式获取到`标量值`
``` rust
fn main() {
    let s=String::from("你好");
    for b in s.chars(){
        println!("{}",b);
    }
}
```
还有一种`字形簇`，得去crate.io中下相关的包获取，标准库中没有获取这个的方法

可以使用`字符串切片`的方式进行获取，但要注意的是，`&s[0..3]`是按照字节数进行切割的，一个`你`占用的字节数为3，所以如果切割的是`0..2`，也就是取第0位和第1位的字符，没有按照字符的边界切割，程序会恐慌
``` rust
fn main() {
    let s=String::from("你好");
    let c=&s[0..3];
    println!("{}",c);
}
```

# HashMap
在rust中,定义一个`hashmap`的方式如下
``` rust
use std::collections::HashMap;

fn main() {
    let mut map=HashMap::new();
    map.insert("a", 1);
}
```

下面的案例演示了如何从`hashmap`中取得数据
``` rust
use std::collections::HashMap;

fn main() {
    let mut scores=HashMap::new();
    let t1=String::from("Blue");
    let t2 =String::from("Red");
    scores.insert(t1, 1);
    scores.insert(t2, 2);
    let tean_name=String::from("Blue");
    let value=scores.get(&tean_name);
    match value {
        Some(v)=>println!("{}",v),
        None=>println!("no this team")
    };
}
```

使用`entry`方法来判断

比如以下这行代码，可以在仅`team_name`这个键值存在的时候`insert`后面的值

本来`.entry(team_name)`返回的是一个`entry`对象

但是`or_insert`会将`entry`对象变为它的value的类型，它的逻辑是：如果当前就值就直接返回value，如果没有就插入一个然后返回value。
``` rust
scores.entry(team_name).or_insert(4);
```
可以用以下代码统计词的数量
``` rust
use std::collections::HashMap;

fn main() {
    let sentence="hello world hello";
    let mut word_count=HashMap::new();
    for word in sentence.split_whitespace(){
        let count=word_count.entry(word).or_insert(0);
        *count+=1;
    }
    println!("{:?}",word_count)
}
```