use std::io;
use std::process::exit;

fn gcd(a: u64,b: u64) -> u64 {
    if b > a {
        gcd(b,a)
    }
    else if b == 0 || a == 0 || (a == 0 && b == 0) {
        println!("Invalid argument to gcd function. The numbers must be non-zero");

        exit(1);
    }
    else {
        gcd(b,a%b)
    }
}


fn main() {
    let mut a = String::new();
    println!("Please enter the first number:");
    io::stdin().read_line(&mut a).expect("Number not entered");
    let a: u64 = a.trim().parse().expect("Please type a number");

    let mut b = String::new();
    println!("Please enter the second number:");
    io::stdin().read_line(&mut b).expect("Number not entered");
    let b: u64 = b.trim().parse().expect("Please type a number");

    
    println!("GCD of {} and {} is {}",a, b, gcd(a,b));
}
