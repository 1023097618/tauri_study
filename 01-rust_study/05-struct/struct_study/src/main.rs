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