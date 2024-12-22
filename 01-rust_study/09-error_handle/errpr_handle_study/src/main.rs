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