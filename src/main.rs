fn main() {
    println!("gcd(128,64) = {}",gcd(128,64));
}

fn gcd(a:u64, b:u64) -> u64 {
    assert!(a!=0 && b!=0 && a>=b) ;
    let c:u64 = a % b;
    if c==0 {
        b
    }
    else {
        gcd(b,c)
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,7),7);

    assert_eq!(gcd(3*7*11*13*19, 2*3*5*11*17),3*11);
}
