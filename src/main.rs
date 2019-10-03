use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}",
             numbers, d);
}
fn gcd(mut a:u64, mut b:u64) -> u64 {
    assert!(a!=0 && b!=0);
    while b!=0 {
        if b<a {
            let t=b;
            b=a;
            a=t;
        }
       b=b%a; 
    }
    a
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,7),7);

    assert_eq!(gcd(3*7*11*13*19, 2*3*5*11*17),3*11);
}
