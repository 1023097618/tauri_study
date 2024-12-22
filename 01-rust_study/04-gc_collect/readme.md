# stack内存和heap内存的差别

存储`stack`内存中的数据必须具有固定长度的大小，而存储在`heap`内存中的数据可以没有固定长度的大小，由操作系统来寻找需要长度的大小并进行分配，因此，将数据存储到`stack`中会快一些

同样的，在读取数据时，如果将数据存储到`stack`中也会快一些，因为操作系统不需要根据指针来查询到相关的数据。

# copy Trait和Drop Trait

1. `在rust中，`copy Trait`指的是这个数据类型是直接存在stack中的，所以在赋值的时候直接复制原来的值，并且原来的变量是可用的。这一般发生在简单数据类型时。
``` rust
let x=1;
let y=x;
println!("x是{},y是{}",x,y);
```
> 如果一个元组中所有的值都实现了`copy Trait`，那么这个元组也会实现`copy Trait`。

``` rust
let tuple:(i32,i32)=(2,3);
let tuple1=tuple;
println!("tuple的第一个值是{}",tuple.0);
```

2. 而`Drop Trait`指的是这个数据类型的值是存在heap中，而对该值的引用的指针存放在stack中，所以它支持通过指针来free相关的内存空间，并且在有`Drop Trait`数据类型的时候赋值的时候运来的值会失效，我们称之为`move`。这个特性一般在复杂数据类型时。

以下代码中,a就是不合法的，因为对String::from("123")的`所有权`已经移动到了b变量中
``` rust
let a=String::from("123");
let b=a;
println!("a是{},b是{}",a,b);
```

如果tuple中有一个值没有实现`copy Trait`，那整个tuple就是`Drop Trait`的，可以得出结论，以下代码是不合法的
>这一点对于array也是一样的，读者有兴趣可以自己尝试

``` rust
let tuple:(i32,String)=(2,String::from("123"));
let tuple1=tuple;
println!("tuple的第一个值是{}",tuple.1);
```

# 函数的所有权转移
1.  形参调用的时候的所有权转移

在rust中，如果对函数的形参做赋值操作，也视为一次赋值，会根据上述的`copy trait`和`drop trait`来判断变量是否失效

比如说以下代码是不合法的，因为所有权发生了转移
``` rust
fn main() {
    let a=String::from("123");
    take_owner(a);
    println!("a的值是{}",a);
}
fn take_owner(x:String){
    println!("x的值是{}",x);
} 

```

2. 作为返回值时的所有权转移
在以下代码中先调用了`take_owner(s)`这个函数，将s赋值给了x,则所有权交到x手上，然后执行到`x`这一行之后，所有权到了a手上，然后在函数执行完，到`}`这一行的时候，`x`并不会被销毁，因为它已经没有了所有权

``` rust
fn main() {
    let s=String::from("string");
    let a=take_owner(s);
}
fn take_owner(x:String)->String{
    x
}
```

# 引用和借用
`引用`可以理解为创建了一个指向指针的指针。而因为这个被指向的指针是没有`drop trait`的，所以我们可以通过这个方式**获得某个值而不取得这个值的所有权**，当这件事发生在函数传参的时候，我们称之为`借用`

以下是一个`借用`的例子，其中s是指向了在堆中"string"的指针，&s创建了一个指向s的指针，它被赋值到了x，因为指针是不会发生所有权转移的，所以这个赋值的过程中只会拷贝一份,所以这也实现了**获得某个值而不取得这个值的所有权**

``` rust
fn main() {
    let s=String::from("string");
    let len=calc_len(&s);
    println!("s的长度是{}",len);
}
fn calc_len(x:&String)->usize{
    x.len()
}
```
>上面的代码是不能对存在堆内存中的String进行改变的，如果要改变，应该采用以下写法，在各个地方都加上mut，声明它是可变的

``` rust
fn main() {
    let mut s=String::from("string");
    let len=calc_len(&mut s);
    println!("{}的长度是{}",s,len);
}
fn calc_len(x:&mut String)->usize{
    x.push_str(" haha!");
    x.len()
}
```

还有以下的规则，这里不一一演示了，主要是为了防止`数据竞争`
1. 不可以同时有一个以上的`可变引用`
2. `不可变引用`可以存在多个
3. `可变引用`和`不可变引用`不能同时存在

还有以下可能产生`悬空引用`的模式也是不被允许的
``` rust
fn main() {
    let x=get_s();
}
fn get_s()-> &String {
    let s=String::from("s");
    &s
}
```

# 字符串切片
字符串切片的类型为`&str`，我们使用字符串切片可以写出`get_first_word`程序
``` rust
fn main() {
    let s1=String::from("Hello world");
    let x=get_first_word(&s1);
    println!("the first word is {}",x);
}
fn get_first_word(s:&str) -> &str{
    let bytes=s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item==b' '{
            return &s[..i];
        }
    }
    &s[..]
}
```

之所以`&str`前面需要加个`&`是为了保持`rust`语法的一致性，如果说返回的是个`str`,那么如果这个`str`被销毁了，那String是不是也要跟着被销毁，那就不符合`字符串切片`这个设计思路了。

以下是不合理代码：
``` rust
fn main() {
    let s1=String::from("Hello world");
    {
        let s2:str=s1[..];
    }
}
```
那么s2离开的时候被销毁了，那s2这个切片指向的堆内存中的地址也需要被销毁，那么s1就访问不到正常的值了，会造成这个指针问题，所以才设计成了`&str`的形式

这么写就对了，这样使用了`借用`，就不会有销毁的问题了
``` rust
fn main() {
    let s1=String::from("Hello world");
    {
        let s2:&str=&s1[..];
    }
}
```