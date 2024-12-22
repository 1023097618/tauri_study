# struct
在rust中，struct的定义和使用如下
``` rust
fn main() {
    let s = Student{
        username:String::from("zs"),
        id:24
    };
    println!("该名学生的姓名是{}",s.username);
}

struct Student{
    username:String,
    id:usize
}
```
同样的，如果struct声明为`mut`，那么它里面的变量都是可变的，否则都是不可变的

在rust中，可以使用以下`解构`的方式快速定义具有相同值的同一结构体的实例

``` rust
fn main() {
    let s = Student{
        username:String::from("zs"),
        id:24
    };
    let s1=Student{
        username:String::from("ss"),
        ..s
    };
    println!("该名学生的姓名是{}",s.username);
}

struct Student{
    username:String,
    id:usize
}
```

同样，struct中每个实现了`drop trait`的成员变量都会有所有权转移的问题

例如、以下代码是不合法的
``` rust
fn main() {
    let s = Student{
        username:String::from("zs"),
        id:24,
        department:String::from("computer science")
    };
    let a=s.username;
    println!("该名学生{}",s.username);
}

struct Student{
    username:String,
    id:usize,
    department:String
}
```

# tuple Struct
`tuple struct`比较适合你不像给struct中的每个字段取名的场景，`tuple struct`由于没有成员变量名，所以访问方式和`tuple`很像，请看以下演示代码
``` rust
fn main() {
    struct Color(i32,i32,i32);
    let s=Color(1,2,3);
    println!("颜色的第一个值是{}",s.0);
}
```
> 还可以使用括号解构的方式来对它进行访问，自己试一试吧！

# 结构体的调试输出
使用`#[derive(Debug)]`可以实现结构体的debug特性，然后使用`{:?}`来输出结构体内部的值，代码如下：
``` rust
fn main(){
    let rec=Rectangle{
        width:12,
        height:10
    };
    println!("长方形是{:#?}",rec);
}
#[derive(Debug)]
struct Rectangle{
    width:usize,
    height:usize
}
```

# 结构体中的方法和关联函数
使用`impl`关键字来定义结构体中的`方法`，调用它的方式是`instance.method()`
``` rust
fn main(){
    let rec=Rectangle{
        width:12,
        height:10
    };
    let area=rec.area();
    println!("长方形的体积是{}",area);
}
struct Rectangle{
    width:usize,
    height:usize
}

impl Rectangle {
    fn area(&self)->usize{
        self.height*self.width
    }
}
```

如果`方法`中的第一个参数类型不为`&self`,我们就将它称之为`关联方法`,调用它的方式是`Struct::function`

代码如下
``` rust
fn main(){
    let rec=Rectangle{
        width:12,
        height:10
    };
    let area=rec.area();
    println!("长方形的体积是{}",area);
    let square=Rectangle::square(3);
    println!("正方形的体积是{}",square.area());
}
struct Rectangle{
    width:usize,
    height:usize
}

impl Rectangle {
    fn area(&self)->usize{
        self.height*self.width
    }

    fn square(length:usize)->Rectangle{
        Rectangle{
            width:length,
            height:length
        }
    }
}
```