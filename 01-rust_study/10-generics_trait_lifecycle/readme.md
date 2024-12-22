# 泛型
这个和我学过C#中的泛型差不多，所以不多赘述，唯一需要注意的点是这种情景

下面的`impl Point<i32>`中的`add`方法仅在`T`为`i32`类型的时候有，其他类型的时候没有。`impl<T> Point<T>`声明此时`impl`内出现`T`与该`Point`结构体中的`T`一致。
``` rust
struct Point<T>{
    x:T,
    y:T
}
impl Point<i32> {
    pub fn add(){
    }
}
impl<T> Point<T> {
    pub fn reduce(){
    }
}
fn main() {
    println!("Hello, world!");
}
```
例子：
``` rust
struct Point<T,Y>{
    x:T,
    y:Y
}

impl<T,Y> Point<T,Y> {
    fn mixed<P,E>(self,other_point:Point<P,E>)->Point<T,E>{
        Point{
            x:self.x,
            y:other_point.y
        }
    }
}
fn main(){
    let p1=Point{
        x:1,
        y:2
    };
    let p2=Point{
        x:'a',
        y:'b'
    };
    let p3=p1.mixed(p2);
    println!("p3.x={},p3.y={}",p3.x,p3.y);
}
```
# Trait
Trait有一些像java中的接口

通过以下语法定义一个trait
``` rust
pub struct Twitter{
    pub content:String,
    pub time_span:usize
}

pub struct WeChat{
    pub content:String,
    pub frind:String
}

pub trait Summary {
    fn summary_content(&self)->String{
        format!("summary...")
    }
}

impl Summary for WeChat{
}

impl Summary for Twitter {
    fn summary_content(&self)->String {
        format!("content:{},timespan:{}",self.content,self.time_span)
    }
}
```
同时呢，trait还支持默认实现
``` rust
pub trait Summary {
    fn summary_content(&self)->String{
        format!("summary...")
    }
}

impl Summary for WeChat{
}
```
此外，不能用外部的trait去实现外部的struct，比如不能拿display这个trait去实现vector，这样做是防止两个不同的包都干了这件事情，然后rust就不知道以谁的为准了。

还可以将实现该接口的类作为一个类型，比如有下面的方法，使用`impl Summary`来声明它
``` rust
pub fn print_summary(item:impl Summary){
    println!("{}",item.summary_content());
}
```

还可以使用这种方式，这与上方的完全等价，实际上**上方这种写法只是下方写法的语法糖**
``` rust
pub fn print_summary<T:Summary>(item:T){
    println!("{}",item.summary_content());
}
```
下面分别是上方两种方式对应的多trait约束的情况，使用`+`来连接多个trait
``` rust
pub fn print_summary<T:Summary+Display>(item:T){
    println!("{}",item.summary_content());
}
```
``` rust
pub fn print_summary(item: impl Summary+Display){
    println!("{}",item.summary_content());
}
```
还可以使用`where`语句来使语句变得更加简洁明了
``` rust
pub fn print_summary<T, U>(item: T, item1: U)
where
    T: Display + Summary,
    U: Clone + Debug,
{
    println!("{}", item.summary_content());
}
```
> 使用impl表示返回类型的时候只能返回一个类型，不能用一个if else返回两种不同的类型

以下是一个例子，使用了泛型参数实现了获取一个数组中最大值的方法
``` rust
fn get_largest<T:PartialOrd>(list:&Vec<T>)->&T{
    let mut a=&list[0];
    for li in list.iter(){
        if a<li{
            a=li;
        }
    }
    a
}
fn main(){
    let v=vec![String::from("aaa"),String::from("bbb")];
    let l=get_largest(&v);
    println!("{}",l);
}
```

还可以在impl中为实现了特定的trait的struct进行方法实现，语法如下
``` rust
impl<T:Copy> Point<T>{
    
}
```

# 生命周期
rust中，生命周期说明了不同变量生效的作用域，前面的课程中，我们写的所有的变量的生命周期都是隐式的自动推断的，如果我们想手动声明生命周期也是可以的。
## 为什么要有生命周期
如果引用比被引用的有更长的生命周期(指更迟被清理)，rust就会报错
``` rust
fn main(){
    let r;
    {
        let x=5;
        r=&x;
    }
    println!("r的值是{}",r);
}
```
对于这个函数
``` rust
fn get_string(a:&str)->&str{
    a
}
```
编译器会自动补全它的生命周期，`'a`代表它的生命周期
``` rust
fn get_string<'a>(a: &'a str) -> &'a str {
    a
}
```
但是对于这个函数，rust会难以知道返回值的生命周期是参考了a的还是参考了b的
``` rust
fn get_string(a:&str,b:&str)->&str{
    a
}
```
如果放任这种定义合法，会出现这种情况，出现`悬垂引用`的问题
``` rust
fn get_string(a: &str, b: &str) -> &str {
    a
}

fn main() {
    let string1 = String::from("hello");
    let string2 = String::from("world");

    let result = get_string(&string1, &string2);
    drop(string1); // string1 被释放
    println!("{}", result); // result 引用了已经被释放的 string1，导致未定义行为
}
```
此时我们就希望，如果返回值的生命周期和`a`一样，上述代码就不予编译通过，但是如果生命周期和`b`一样，我们就允许它的代码通过编译。

## 函数中的生命周期
所以就有了我们正确的规定生命周期的方法
``` rust
fn get_string<'a, 'b>(a: &'a str, b: &'b str) -> &'a str {
    a
}
```
以下是一个例子，返回值比较长的字符串的引用。
``` rust
fn get_largest_number<'a>(a:&'a str,b:&'a str)->&'a str{
    if a.len()>b.len() {
        a
    }else{
        b
    }
}

fn main() {
    let a="aa";
    let b="lala";
    let c=get_largest_number(a, b);
    println!("{}",c);
}

```
需要注意的是，这边的`'a`指代的形参a和形参b中最早被清除的那一个的生命周期。换句话说，传进来的形参a和形参b的生命周期不必一样。

以下代码也是可以通过编译的
``` rust
fn get_largest_number<'a>(a:&'a str,b:&'a str)->&'a str{
    if a.len()>b.len() {
        a
    }else{
        b
    }
}

fn main() {
    let a=String::from("aa");
    let c;
    {
        let b=String::from("bb");
        c=get_largest_number(a.as_str(), b.as_str());
        println!("{}",c);
    }
    let d=1;
}
```

不能返回函数内部创建的值的引用
``` rust
fn get_largest_number<'a>(a:&'a str,b:&'a str)->&'a str{
    let c=String::from("ddd");
    c.as_str()
}
```
需要换一种设计思路，直接返回`String`类型
``` rust
fn get_largest_number<'a>(a:&'a str,b:&'a str)->String{
    String::from("ddd")
}
```
> 生命周期是针对引用而言的，而对于String这类值，rust是通过所有权来保证它的安全的

## 结构体中的生命周期
我们可以通过声明周期为一个定义一个结构体，语法如下
``` rust
struct Part<'a>{
    x: &'a str
}

fn main() {
    let s=String::from("abc");
    let p=Part{
        x:s.as_str()
    };
}
```

当我们使用`impl`为一个有生命周期的struct定义方法时，语法如下
``` rust
struct Part<'a>{
    x: &'a str
}

impl<'a> Part<'a> {
    fn get_x(&self){
        println!("{}",self.x)
    }
}
```
结构体中的`方法`(注意不是`关联函数`)若返回的是一个引用，可以不为它指明生命周期，它的生命周期和`&self`是一样的，这么写是对的
``` rust
struct Part<'a>{
    x: &'a str
}

impl<'a> Part<'a> {
    fn get_x(&self,b:&str)->&str{
        self.x
    }
}
```
补全就是这样
``` rust
impl<'a> Part<'a> {
    fn get_x<'b>(&'a self,b:&'b str)->&'a str{
        self.x
    }
}
```

## `'static`生命周期
`'static`生命周期是整个程序的运行时间，像之前我们指定字符串的值
``` rust
let b="123";
```
补全了就是
``` rust
let b:&'static str="123";
```