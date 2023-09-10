enum Pet {dog, cat, fish}

impl Pet {
    fn get_name(&self) -> &'static str   {
        match self {
            Pet::dog => "Dog",
            Pet::cat => "Cat",
            Pet::fish => "Fish",
        }
    }
}

enum IpAddrKind {
    V4(String),
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    println!("Hello, world!");

    let pet = Pet::dog;
    println!("Pet name is: {}", pet.get_name());

    let _home = IpAddrKind::V4(String::from("127.0.0.1"));

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };    
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("The value of six is: {:?}", six);
    println!("The value of none is: {:?}", none);

    what_pet(Pet::dog);
    what_pet(Pet::cat);
    what_pet(Pet::fish);

    let dog2 = Some(Pet::dog);
    if let Some(Pet::dog) = dog2 {
        println!("It's a dog");
    } else {
        println!("It's not a dog");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("The value of top is: {}", top);
    }

    let mut x = 1;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Unknown"),
    }

    match x {
        1 | 2 => println!("One or Two"),
        5..=10 => println!("Five to Ten"),
        3 => println!("Three"),
        _ => println!("Unknown"),
    }

    let v = Some(5);
    let u = 5;
    match v {
        Some(10) => println!("Ten"),
        Some(x) if x == u => println!("Five"),
        _ => println!("Unknown"),
    }


}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn what_pet(pet: Pet) {
    match pet {
        Pet::dog => println!("Pet is a dog"),
        Pet::cat => println!("Pet is a cat"),
        Pet::fish => println!("Pet is a fish"),
        _ => println!("Pet is unknown"),
    }
}