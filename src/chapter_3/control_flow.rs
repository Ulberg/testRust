fn my_func() -> i32 {
    println!("LOL");
    secondary();
    return 23;
}

fn if_block() {
    let x = if (32 > 23) { 32 } else { 2 };
}

fn infinite_loop() {
    let mut counter = 0;
    loop {
        counter += 1;
        if (counter > 10) {
            break;
        }
    }
}
fn while_lopp() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
}
fn for_in_loops() {
    let a = [32, 2, 42, 1, 4, 5, 92];
    for element in a.iter() {
        println!("{}!", element);
    }
    for number in 1..15 {
        println!("{}!", number);
    }
}
fn secondary() {
    let x = 5;
    println!(" The value of x is: {}", x);
    let x = "six";
    println!("The value of x is now : {}", x);

    const SUBSCRIBER_COUNT: u32 = 10_000;

    // Bool
    let t = true;
    let f = false;
    // Char
    let c = 'c';

    // complex types
    // tuples
    let tup = ("str", 100_000);
    let (channel, sub_count) = tup;
    let sub_count_2 = tup.1;
}
