fn main() {
    let s1=String::from("Hello world");
    {
        let s2:&str=&s1[..];
    }
}