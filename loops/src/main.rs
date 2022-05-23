fn main() {
    let mut count = 0;
    'asdf: loop { //loop label
        count = count+1;
        println!("Again!");
        if count == 10 {
            break;
        }
        let mut count2 = 0;
        loop {
            count2 = count2+1;
            println!("Inner loop!");
            if count2 == 5 {
                break 'asdf; // WHOAT YOU CAN NAME LOOPS???
            }
        }
    }

    let result = loop {
        count = count+1;
        println!("Again!");
        if count == 10 {
            break count*2;
        }
    };

    println!("Result: {}", result);

    let mut i = 0;
    while i < 5 {
        println!("{}", i);
        i = i+1;
    }

    let a: [i32; 5] = [3, 4, 5, 6, 7];
    
    i = 0;

    while i < a.len() {
        println!("{}", a[i]);
        i = i+1;
    }
    
    
    for element in a.iter() {
        println!("{}", element);
    }

    for element in a {
        println!("{}", element); //what??
    }

    // println!("{}", (0..5).sum::<i32>());

    for i in 0..50 {
        println!("asdf{}", i);
    }

    for i in (0..50).rev() {
        println!("{}", i);
    }

    fibonacci(500);

}

fn fibonacci(n: u128) {
    println!("Fibonacci till {}", n);
    let mut a: u128 = 1;
    let mut b: u128 = 1;
    for i in 0..n {
        let c: u128 = a+b;
        a = b;
        b = c;
        println!("{}", a);
    }
}