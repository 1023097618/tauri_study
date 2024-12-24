class Person<T extends string>{
    name:T;
    constructor(name:T){
        this.name=name
    }
}
const p=new Person<string>("1");
