use procmacros::debug_print;

macro_rules! average {
    ($(,)*) => {{
        0.0
    }};

    ($( $x:expr ),+ $(,)*) => {{
        let count = 0usize $(+ { let _ = stringify!($x); 1 })*;
        let sum = 0.0 $(+ $x as f64)*;
        sum / count as f64
    }};
}

#[debug_print]
fn main() {
    println!("Hello, world!");
    println!("{}", average!());
    println!("{}", average!(2.0, 4.0, 6.0));
    println!("{}", average!(1, 2, 3, 4, 5));
}
