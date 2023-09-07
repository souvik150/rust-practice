fn main() {
    let x: i8 = 10;
    println!("The value of x is: {}", x);

    let _y: u8 = 10;

    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("The value of decimal is: {}", decimal);
    println!("The value of hex is: {}", hex);
    println!("The value of octal is: {}", octal);
    println!("The value of binary is: {}", binary);

    let byte = b'A';
    println!("The value of byte is: {}", byte);

    let _x = 2.0;
    let _y: f32 = 3.0;

    let _t = true;
    let _f: bool = false;

    let c = 'c';
    println!("The value of c is: {}", c);

    // + - * / %

    let a = 10;
    let b = 4;

    let remainder = a % b;
    println!("The value of remainder is: {}", remainder);

    let tup = (500, "hi", true);
    println!("The value of tup is: {}", tup.0);
    println!("The value of tup is: {}", tup.1);
    println!("The value of tup is: {}", tup.2);

    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let mut arr = [1, 2, 3, 4, 5];
    println!("The value of arr is: {}", arr[0]);
    println!("The value of arr is: {}", arr[1]);
    println!("The value of arr is: {}", arr[2]);
    println!("The value of arr is: {}", arr[3]);
    arr[4] = 6;
    println!("The value of arr is: {}", arr[4]);

    let mut nums = vec![1, 2, 3, 4, 5];

    nums.push(6);

    println!("The value of nums is: {}", nums[0]);
    println!("The value of nums is: {}", nums[5]);

    println!("The value of nums is: {:?}", nums);

    nums.pop();

    println!("The value of nums is: {:?}", nums);

    let mut vec = Vec::new();
    vec.push("Test");
    vec.push("String");

    println!("The value of vec is: {:?}", vec);

    vec.reverse();
    println!("The value of vec is: {:?}", vec);

    let vect = Vec::<i32>::with_capacity(2);
    println!("{}", vect.capacity());

    let v: Vec<i32> = (0..5).collect();
    println!("The value of v is: {:?}", v);

    let sv: &[i32] = &v[2..4];
    println!("The value of sv is: {:?}", sv);

    let name = String::from("John Doe");
    println!("The value of name is: {}", name);

    let mut s = String::new();
    s.push_str("Hello, ");
    s.push_str("World!");
    println!("The value of s is: {}", s);

    let name = String::from("John Doe");
    let course = "Rust".to_string();

    let message = format!("{} is learning {}", name, course);
    println!("The value of message is: {}", message);

    // string slice
    let str1 = "Hello, World!";
    let str2 = &str1[0..5];
    println!("The value of str2 is: {}", str2);

    let str3 = "Hello, World!";
    let str4 = &str3[0..=4];
    println!("The value of str4 is: {}", str4);

    println!("{}", "ONE".to_lowercase() == "one");

    let rust = "\x52\x75\x73\x74";
    println!("The value of rust is: {}", rust);
}
