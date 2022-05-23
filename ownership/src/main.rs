fn main() {
    println!("Hello, world!");

    let s = "hello";
    println!("{}", s);
    let mut s2 = String::from("hello");
    s2.push_str(", world!");
    println!("{}", s2);
    drop(s2);
    // println!("{}", s2); // error: will not worked since we drop s2
    

}

