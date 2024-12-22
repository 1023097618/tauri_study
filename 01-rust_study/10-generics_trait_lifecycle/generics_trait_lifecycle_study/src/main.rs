struct Part<'a>{
    x: &'a str
}

impl<'a> Part<'a> {
    fn get_x<'b>(&'a self,b:&'b str)->&'a str{
        self.x
    }
}

fn main() {
    let a: &str="123";
    let s=String::from("abc");
    let p=Part{
        x:s.as_str()
    };
    let b:&'static str="123";
}
