fn main() {
    println!("gcd(128,64) = {}",gcd(128,64));
    println!("gcd(64,128) = {}",gcd(64,128));
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
