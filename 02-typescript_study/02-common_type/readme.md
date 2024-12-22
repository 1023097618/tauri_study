# TypeScript中的基本类型
## 简单对象
使用以下方法可以声明一个有类型的变量
``` typescript
let a:number;
```
后续赋值操作和js相同，如果给这个变量赋值不为number的时候会报错

还有`string`类型
``` typescript
let a:number=123;
let b:string="123";
```
当在声明变量的时候同时对它进行初始化，ts会自动完成类型推断，无需手动指明类型，所以下方的代码会报错
``` typescript
let c=false;
c=123;
```
还可以对函数参数和返回值做类型声明
``` typescript
function sum(a:number,b:number):number{
    return a+b;
}
console.log(sum(1,2));
```
还支持联合类型声明，声明的时候使用`|`，支持使用字面量，这个例子里，c只能是"`male"`或`"female"`或`boolean`
``` typescript
let c:"male"|"female"|boolean;
c="male";
console.log(c)
c=false;
console.log(c);
```
当类型为`any`的时候，赋值给任何的类型都不报错
``` typescript
let a:any=true;
let b:string=a;
console.log(b);
```
当类型为`unknown`的时候，赋值给任何的类型(除了unknown和any)都不行
``` typescript
let a:unknown="abc";
let b:string=a;
```
几种将`unknwon`不报错赋值给其他类型的方式，比如下列代码不会报错
```typescript
b=a as string;
b=<string>a;
if(typeof a === "string"){
    b=a;
};
```
>当函数的返回值设置为never的时候，一般这是个报错的函数

## 复杂对象
### 对象类型定义方式
使用以下方式可以限制对象一定要有特定值，比如下方设定了对象一定要有`name`，`age`属性可以有可以没有，其他属性都不能有。
``` typescript
let obj:{name:string,age?:number};
obj={
    name:"小明"
}
```
使用以下语法指定允许很多其他的属性名定义进来，但是一定要是`键:string -> 值:any`
``` typescript
let obj:{name:string,age?:number,[propName:string]:any};
obj={
    name:"小明",
    departMent:"计算机学院"
}
```
使用`&`来表示要同时满足多个obj的条件
``` typescript
let obj:{name:string}&{age:number};
obj={
    name:'小明',
    age:12
}
```

### 函数类型定义方式
下面的代码定义了一个有且仅有两个`number`类型的参数，并且返回值也为`number`的函数类型
``` typescript
let d:(a:number,b:number)=>number;
d=((a,b)=>{
    return a+b
})
```

### 数组类型定义方式
有以下的两种语法定义它
``` typescript
let a:string[];
a=["123","321"];
let b:Array<string>;
b=["1","2"];
```

### 元组类型定义方式
相比于javascript，这属于新加入的类型,其实就是一个定长，定类型的数组
比如以下代码定义了一个第一个值为string，第二个值为number的数组
``` typescript
let a:[string,number];
a=["小明",1234];
```

### enum类型定义方式
其实这个类型在js中也是没有的，在转化为js代码的时候ts会把它转化成定义的值，比如这里female就是0，male就是1
``` typescript
enum Gender{
    Female,
    Male
};
let person:{
    name:string,
    gender:Gender
};
person={
    name:"小明",
    gender:Gender.Male
};
```
## 给类型起别名
有时候类型太长了不方便书写，可以自定义一个自己的类型
``` typescript
type mytype=1|2|3|4
let a:mytype;
a=4;
```