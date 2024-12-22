# rust中处理错误的两种方式
## 使用pani处理错误
### panic概念
rust中有两种处理`panic`的方式

1. 展开：当遇到panic时，程序会沿着调用栈往回走，清理每个函数中的数据，并停止运行(默认)
2. 中止：遇到panic时，程序立刻停止运行，数据交给操作系统进行清除(需要配置，这么做会让打包后的程序体积小一些)

设置`中止`的错误处理方式需要打开`Cargo.toml`,输入以下配置信息
```
[profile.release]
panic='abort'
```

### 如何触发panic
使用`panic!`宏来触发
``` rust
fn main() {
    panic!("crash and burn");
}

```
以下程序可以实现对打开文件的错误处理,虽然说match表达式不同分支需要返回的是相同的类型，但是panic就没关系，能过编译
``` rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt");
    let f=match f{
        Ok(value)=>value,
        Err(error)=>{
            panic!("打开文件失败");
        }
    };
}
```
将上述代码写得复杂些，加上不存在文件就创建的逻辑，代码如下(看起来有点繁琐，后面会用`闭包`来简化这部分代码)
``` rust
use std::{fs::File, io::{ErrorKind,Error}};

fn main() {
    let f: Result<File, Error> = File::open("hello.txt");
    let f: File=match f{
        Result::Ok(value)=>value,
        Result::Err(error)=>match error.kind() {
            ErrorKind::NotFound=>{
                let f: Result<File, Error>=File::create("hello.txt");
                match f {
                    Result::Ok(value)=>value,
                    Result::Err(error)=>panic!("创建文件错误{:?}",error)
                }
            },
            other_error=>panic!("无法打开hello.txt{:?}",other_error)
        }
    };
    println!("{:?}",f);
}
```
可以使用`unwrap`方法来简化调用逻辑

此处`unwrap`的作用是，如果是Result::Ok，那么就返回ok中的值回来，如果是Result::Err，那么就引起`panic`
``` rust
let f=File::open("hello.txt").unwrap();
```
使用`expect`来自定义错误信息
``` rust
let f=File::open("hello.txt").expect("无法打开文件hello.txt");
```
## 使用Result来处理错误
例如:

书写如下信息来获取`hello.txt`中的信息并进行打印
``` rust
use std::{fs::File, io::{Error, Read}};

fn read_string()->Result<String,Error>{
    let f=File::open("hello.txt");
    let mut f = match f{
        Ok(value)=>value,
        Err(err)=>return Err(err)
    };
    let mut s=String::new();
    match f.read_to_string(&mut s){
        Ok(_)=>Ok(s),
        Err(err)=>Err(err)
    }
}
fn main(){
    match read_string(){
        Ok(value)=>println!("值为{}",value),
        Err(_)=>println!("打开文件失败")
    }
}
```
该代码与以下代码等价，也就是说`?`相当于在失败的时候会return失败的结果
``` rust
use std::{fs::File, io::{Error, Read}};

fn read_string()->Result<String,Error>{
    let mut f=File::open("hello.txt")?;
    let mut s=String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
fn main(){
    match read_string(){
        Ok(value)=>println!("值为{}",value),
        Err(_)=>println!("打开文件失败")
    }
}
```
>上面代码中，如果返回的错误类型不为`std::io::Error`，则该错误类型必须实现一个`from`函数，在java中应该可以理解为是继承自这个`std::io::Error`才行

使用链式调用继续优化以上代码
``` rust
use std::{fs::File, io::{Error, Read}};

fn read_string()->Result<String,Error>{
    let mut s=String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
fn main(){
    match read_string(){
        Ok(value)=>println!("值为{}",value),
        Err(_)=>println!("打开文件失败")
    }
}
```

通过修改main函数的返回类型可以进一步简化代码
``` rust
use std::{fs:: File, io::{Error, Read}};

fn read_string()->Result<String,Error>{
    let mut s=String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
fn main()->Result<(),Box<dyn std::error::Error>>{
    let s=read_string()?;
    println!("读出文件的值是{}",s);
    Ok(())
}
```
其中返回`Result<(),Box<dyn std::error::Error>>`后面会讲到

以下是一个例子，描述了需要`panic`的一个场景
``` rust
struct Guess{
    value:i32
}
impl Guess {
    pub fn new(value:i32)->Guess{
        if value<1 || value>100{
            panic!("输入的值不合法");
        };
        Guess{value}
    }
    pub fn value(&self)->i32{
        self.value
    }
}
fn main(){
    let g=Guess::new(1);
    println!("guess的值是{}",g.value());
}
```