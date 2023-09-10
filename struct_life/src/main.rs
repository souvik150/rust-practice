struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}

struct Coordinate(i32, i32);

struct UnitStruct;

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Square) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn whats_my_width(&self) -> u32 {
        self.width
    }

    fn whats_my_height(&self) -> u32 {
        self.height
    }

    fn change_width(&mut self, width: u32) {
        self.width = width;
    }
}

struct MyString<'a> {
    string: &'a str,
}

fn main() {
    let user1 = User {
        email: String::from("souvikmukherjee150.com"),
        username: String::from("souvik150"),
        active: true,
        sign_in_count: 0,
    };

    println!("The value of user1 is: {:?}", user1.username);

    let user2 = build_user(String::from("test.com"), String::from("test"));

    println!("The value of user2 is: {:?}", user2.username);

    let cords = Coordinate(10, 20);
    println!("The value of cords is: {:?}", cords.0);

    let unit = UnitStruct;

    let mut square = Square {
        width: 10,
        height: 20,
    };

    println!("The area of square is: {}", square.area());

    println!("Change width of square");
    square.change_width(30);

    println!("The area of square is: {}", square.area());

    println!("The width of square is: {}", square.whats_my_width());


    let r;
    {
        let x = 5;
        r = &x; //x is dropped here
    }

    //println!("The value of r is: {}", r);
    
    //&i32
    //&'a i32
    //&'a mut i32

    let str1 = String::from("Hello, World!");

    //need to ensure MyString does not outlive str1 because MyString holds a reference to str1 else str1 will be dropped and MyString will be left with a dangling reference
    let x = MyString { string: str1.as_str() };
    


}


fn example<'a>(x: &'a str) -> &'a str {
    x
}