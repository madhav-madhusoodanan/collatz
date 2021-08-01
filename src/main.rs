fn main() {
    println!("collatz conjecture testing \n");
    let mut i:u128 = 3;
    loop {
        if i == 1_000_000_000 {
            break;
        }
        println!("\nFor {}, it is {}", i, collatz(&i));
        i += 1;
    }
}

fn collatz(&num1: &u128) -> bool {
    let mut num = num1;
    loop {
        if num == 4 {
            return true ;
        }
        if num % 2 == 0 {
            num = num / 2;
        }
        else {
            num = num * 3 + 1;
        };
}
}
