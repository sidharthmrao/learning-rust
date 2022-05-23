fn main() {
    println!("Hello, world!");

    another_function(5, "Hello", 'h', 6);
}

/*asdf
asdf
asdf
*/

fn another_function(x: i32, y: &str, z: char, a: i32) -> i32 { // regular comment
    println!("Another function: {} {} {}", x, y, z);
    let b = {
        let c: bool = false;
        if c {
            x + a
        } else if !c {
            x - a
        } else {
            x * a
        }
    };
    println!("{}", b);

    let c = true;
    let b = if c {x+a} else if !c {x-a} else {x*a}; //can't have a string in one and int in another
    return b;
}
