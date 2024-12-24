# 类
## 定义类
ts中的类的定义方法如下，对于`static`关键字可以通过类来访问，否则只能通过示例对它进行访问
``` ts
class Person{
    name:string="123"
    age:number=12
    static pname:string="ppp"
}
const p=new Person()
console.log(p)
console.log(Person.pname)
```
>方法也是一样，直接定义在其中即可，此处省略
## 指定构造函数
使用以下函数指定类的构造方法
``` ts
class Person{
    name:string="123"
    age:number=12
    constructor(name:string,age:number){
        this.name=name;
        this.age=age
    }
}
const p=new Person("1",1);
console.log(p);
```
> 中间还讲了类的继承，super关键字、抽象类，此处略过
## 接口定义类
使用接口来定义一个类，语法如下
``` ts
interface Person{
    userName:string,
    sayHello():void
}
const person:Person={
    userName:"123",
    sayHello(){
        console.log("11223");
    }
}
person.sayHello();
```

## 属性封装
还可以对属性进行封装，主要使用`private`、`public`、`get`、`set`关键字，语法如下

``` ts
class Person{
    private _name:string
    private _age:number
    constructor(name:string,age:number){
        this._name=name
        this._age=age
    }
    get name(){
        return this._name;
    }
    set name(name){
        this._name=name
    }
}
const p=new Person("1",1);
p.name="小张";
console.log(p.name)
```

## 泛型
ts同时也支持泛型，对于方法调用时候的语法如下
``` ts
function print_value<T>(a:T){
    console.log(a)
}
print_value<string>("haha")
```
对于类语法如下
``` ts
class Person<T>{
    name:T;
    constructor(name:T){
        this.name=name
    }
}
const p=new Person<string>("1");
```
>同时也可以在声明泛型的时候使用 `T extends xxx`来实现对泛型的约束