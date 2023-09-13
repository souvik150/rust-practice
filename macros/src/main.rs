macro_rules! gcd {
    ($x:expr, $y:expr) => {
        {
            let mut a = $y;
            let mut b = $x;

            while a != 0 {
                if a < b {
                    let t = a;
                    a = b;
                    b = t;
                }
                a = a % b;
            }
            b
        }
    };
}

fn main() {
    println!("{}", gcd!(14, 16));
}
