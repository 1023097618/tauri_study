这一节主要学习了rust中的`变量`、`常量`、`tuple`、`array`、`函数`、`if语句`、`循环方式`

# 变量
变量可以分成`不可变变量`和`可变变量`

`不可变变量`声明方式如下
``` rust
let a:i32=1;
```
对于不可变变量而言，这么写编译器会报错
``` rust
let a:i32=1;
a=2;
```
但是可以通过重新声明的方式覆盖(**shadow**)掉原先的变量

方式如下
``` rust
let a="1234";
let a=a.len();
```


`可变变量`声明方式如下
``` rust
let mut a:i32=1;
```
虽然`可变变量`之后可以重新赋值，但是类型不能改变
这么写是不合法的
``` rust
let mut a="1234";
a=a.len();
```

---
# 常量
对于常量来讲，声明方式如下
``` rust
const MAX_POINT:i32 = 123;
```

---
# 此外
rust中的`char`类型与C语言有差异，rust的char类型支持中文还有一些奇怪的字符，类似于`😂`，它占用4个字节。

# tuple
tuple中各个元素的类型可以不同

使用以下方式定义tuple
``` rust
let tup:(i32,i64,f32)=(1,2,2.0);
```
使用以下方式访问tuple

方式1
``` rust
println!("{},{},{}",tup.0,tup.1,tup.2);
```

方式2
``` rust
let (a,b,_)=tup;
```
>注意：tuple一旦定义之后就是定长不能改变

# array
array中数组中各个元素的类型应该相同，并且存在栈中

使用以下方式声明，该方法表示声明的数组存储的类型为i32，且长度为3
``` rust
let array:[i32;3]=[1,2,3];
```

或者采用以下方法声明，该方法表示声明的数组存储的值均为3，且长度为5
``` rust
let array1=[3;5];
```
使用以下方式访问数组，和其他语言一样
``` rust
array[0]
```

# 函数
使用以下方式可以声明一个函数，需要注意的是，函数形参(parameter)的类型必须显式指明
``` rust
fn another_function(x:i32,y:i32){
    println!("x是{}",x);
    println!("y是{}",y);
}
```
在rust中，如果需要让函数有返回值，则在函数的`语句`结尾必须有一个`表达式`,并且必须声明返回的类型，比如
``` rust
fn plus_five(x:i32)->i32{
    x+5
}
```
在这个例子中，`x+5`结尾没有`;`，则`x+5`是一个`表达式`而不是`语句`

# if语句
在rust中，if语句和其他语言是差不多的，但有一点不同，就是可以通过if语句返回相关的值，并且返回的值的类型必须是相同的
``` rust
let ifnum=60;
let ifresult=if ifnum<65{
    3
}else{
    4
};
println!("ifresult的值是{}",ifresult);
```

# 三种循环方式
在rust中，循环的语句主要有`loop`还有`while`
其中`loop`可以通过`break`语句返回一个值
``` rust
let mut count=0
let loopresult=loop {
    count=count+1;
    if(count==10){
        break count
    }
};
```

`while`的写法与其他语言基本一致

还有使用`for`来遍历数组的，相关写法如下
```rust
let arr=[1,2,3];
for element in arr.iter(){
    println!("element的值是{}",element)
}
```
其中`element`是对arr中的元素的引用，而不是复制一份
