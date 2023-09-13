use core::ops::*;

#[derive(Debug)]
struct Item {
    name: String,
}

// struct City {
//     city : String,
//     population : u64,
// }

// fn sort_pop(city: &mut Vec<City>) {
//     city.sort_by_key(pop_helper)
// }

// fn pop_helper(city: &City) -> u64 {
//     city.population
// }

// fn sort_pop_closure(city: &mut Vec<City>) {
//     city.sort_by_key(|city| city.population);
// }

fn check_invetory(item: Vec<Item>, product: String) -> Vec<Item> {
    item.into_iter().filter(|x| x.name == product).collect()
}

fn main(){
    // let a = City {city: "a".to_string(), population: 470};
    // let b = City {city: "b".to_string(), population: 340};
    // let c = City {city: "c".to_string(), population: 200};
    
    // let mut vec: Vec<City> = vec![a, b, c];
    // sort_pop_closure(&mut vec);

    // for i in vec {
    //     println!("{}: {}", i.city, i.population);
    // }

    // Closures are fast as not allocated on heap and can be inlined
    // so faster than functions and iterators

    let add = | x: i32, y: i32 | -> i32 { x + y };
    println!("{} + {} = {}", 2, 3, add(2, 3));


    //Fn - immutable borrow
    //FnMut - mutable borrow
    //FnOnce - takes ownership

    // Fn is subtarit of FnMut and FnMut is subtrait of FnOnce
    // so if FnOnce is implemented, Fn and FnMut are also implemented

    // || drop(x) - FnOnce
    // |args| v.contains(args) - Fn
    // |args| v.push(args) - FnMut
    

    // let y = 5;
    // let add_y = |x| x + y;
    // let copy = add_y; //this is the closure being copied
    // println!("{}", copy(5));

    // let vec = vec![1, 2, 3];
    // let vec2 = vec![4, 5, 6];
    // let mut vec3 = vec![7, 8, 9];

    // let mut add_vec = |x: Vec<i32>| {
    //     vec3.extend(x);
    //     vec3.push(10);
    //     println!("{:?}", vec3);
    // };

    // add_vec(vec);

    let mut vec : Vec<Item> = Vec::new();
    vec.push(Item {name: "a".to_string()});
    vec.push(Item {name: "b".to_string()});
    vec.push(Item {name: "c".to_string()});
    vec.push(Item {name: "d".to_string()});

    let check = check_invetory(vec, "a".to_string());
    println!("{:?}", check);

    let range = Range {start: 0, end: 10};

    // range is moved into the for loop and is consumed by it in the process of iteration and is dropped at the end of the loop when it goes out of scope of the loop itself and is no longer available. That is the reason is it commented to run below code.
    
    // for i in range {
    //     println!("{}", i);
    // }

    let vec: Vec<u32> = range.filter(|x| x % 2 == 0).collect();
    println!("{:?}", vec);
}

