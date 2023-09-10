fn main() {
    let _var = 1; //created on the stack
    let mut s = "hello".to_string(); //created on the heap


    s.push_str(", world!"); // can increase size of string on heap


    // move ownership

    let x = vec![1, 2, 3]; // created on the heap
    let y = x; // x is moved to y, x is no longer valid
    // println!("{:?}", x); // x is no longer valid
    println!("{:?}", y); // y is valid

    //=========================================================

    // A string is a non-Copy type, so it's moved instead of copied
    let s = String::from("hello"); // created on the heap
    takes_ownership(s); // s is moved into the function
    // println!("{}", s); // s is no longer valid

    //=========================================================

    // An i32 is a Copy type, so it's copied instead of moved
    let val = 1; // created on the stack
    makes_copy(val); // val is copied into the function

    //=========================================================

    let str1 : String = gives_ownership(); // created on the heap
    println!("{}", str1);

    //=========================================================

    let str3: String = take_and_give_back(str1); // str1 is moved into the function and moved back out
    println!("{}", str3);

    //=========================================================

    // clone ownership

    let x = vec![1, 2, 3]; // created on the heap
    let y = x.clone(); // x is cloned to y, x is still valid
    println!("{:?}", x); // x is still valid
    println!("{:?}", y); // y is valid


    //=========================================================

    let mut str1 = String::from("Hello, World!");
    let mut str2 : String;

    loop {
        str2 = str1.clone();
        println!("{}", str2);
        str1.push_str("!");
        if str1.len() > 20 {
            break;
        }
    }

    let mut s = String::from("Hello, World!");
    // pass a mutable reference to the function
    change_string(&mut s);

    println!("{}", s);

}

fn takes_ownership(s : String){
   println!("{}", s); 
}

fn makes_copy(val : i32){
    println!("{}", val);
}

fn gives_ownership() -> String {
    let s = String::from("hello"); // created on the heap
    s // s is returned and moved out to the calling function
}

fn take_and_give_back(s : String) -> String {
    s // s is returned and moved out to the calling function
}

fn change_string(s : &mut String) {
    //borrowed reference
    s.push_str("!");
    println!("{}", s);
}

// var is dropped here, s is dropped here
