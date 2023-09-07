fn main() {
    print_phase("Hello, world!");
    println!("{}", gcd(20, 4));
}

fn print_phase(phrase : &str) {
    println!("The value of phrase is: {}", phrase);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }

        a = a % b;
    }
    b
}  