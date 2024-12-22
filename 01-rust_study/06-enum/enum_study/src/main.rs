fn main() {
    let u: Option<usize>=Some(3);
    if let Some(3)=u{
        println!("three");
    }else{
        println!("others");
    };
}
