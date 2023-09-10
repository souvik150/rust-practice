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

    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    
    
    
}
