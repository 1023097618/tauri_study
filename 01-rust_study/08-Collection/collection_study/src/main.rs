use std::collections::HashMap;

fn main() {
    let sentence="hello world hello";
    let mut word_count=HashMap::new();
    for word in sentence.split_whitespace(){
        let count=word_count.entry(word).or_insert(0);
        *count+=1;
    }
    println!("{:?}",word_count)
}