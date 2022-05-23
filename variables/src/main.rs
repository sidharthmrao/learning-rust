fn main() {
    let x = 5;
    println!("The value of x was: {}", x);
    let x = x+1; // (doesn't work)
    println!("The value of x is: {}", x);
    let mut y = 10;
    println!("The value of y was: {}", y);
    y = 11;
    println!("The value of y is: {}", y);

    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The number of spaces is: {{}} {}?", guess);

    let _guess: u32 = 0xff; // Hex // Underscore means it's an unused variable to avoid issues in compiler
    let _guess: u32 = 0o77; // Octal
    let _guess: u32 = 0b1111_1111; // Binary
    let guess: u8 = b'A'; // Byte
    println!("The number of spaces is: {}.", guess);

    let _x = 2.1; // f64
    let x: f64 = 2.3; // f64
    println!("Float: {}", x);

    let t = true;
    let f: bool = false;
    println!("Boolean: {}", t);
    println!("Boolean: {}", f);

    let c = 'z';
    let z: char = 'y';
    // let y: char = "a"; // errs because you need single quotes
    println!("Character: {}", c);
    println!("Character: {}", z);

    let mut tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    println!("The value of tup[1] is: {}", tup.1);

    tup.0 = 1000;
    println!("The value of tup[0] is: {}", tup.0);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("Months[0]: {}", months[0]);

    let a:[i32; 5] = [1, 2, 3, 4, 5];
    println!("a[0]: {}", a[0]);

    let a = [3; 5];
    // Print all values in a in one line
    println!("a: {:?}", a);

    println!();
}