const MAX_POINT:i32 = 123;
fn main() {
    println!("{}",MAX_POINT);
    let x= "1234";
    let x=x.len();
    let y=123;
    let z=2.0;
    println!("{}",x);
    let h='😂';
    println!("{}",h);

    let tup:(i32,i64,f32)=(1,2,2.0);
    let mut tup1:(i32,i64,f32)=(1,2,2.0);
    tup1.0=2;
    println!("{}",tup1.0);
    let (a,b,_)=tup;
    println!("{},{},{}",tup.0,tup.1,tup.2);
    println!("{},{}",a,b);
    let array:[i32;3]=[1,2,3];
    let array1=[3;5];
    println!("{}",array[0]);
    another_function(array[0], 2);
    let mut num:i32=55;
    num=plus_five(num);
    println!("经过函数加法之后num值为{}",num);
    let ifnum=60;
    let ifresult=if ifnum<65{
        3
    }else{
        4
    };
    println!("ifresult的值是{}",ifresult);
    let mut count=0;
    let loopresult=loop {
        count=count+1;
        if(count==10){
            break count
        }
    };
    println!("loopresult的值是{}",loopresult);

    let arr=[1,2,3];
    for element in arr.iter(){
        println!("element的值是{}",element)
    }
    let ok=(1..3);
    for element1 in ok.rev(){
        println!("element1的值是{}",element1)
    }
}

fn another_function(x:i32,y:i32){
    println!("x是{}",x);
    println!("y是{}",y);
}

fn plus_five(x:i32)->i32{
    x+5
}