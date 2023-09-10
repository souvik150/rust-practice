use rand::seq::SliceRandom;
use rand::thread_rng;

// =====================================

use std::collections::BinaryHeap;

// =====================================

use std::collections::HashSet;

// =====================================

use std::collections::HashMap;

fn main() {

    //===========================================================
    // Vectors
    //===========================================================

    let mut nums:Vec<i32> = vec![];

    nums.push(1);
    nums.push(2);
    nums.push(3);
    nums.push(4);
    nums.push(5);

    let pop = nums.pop();
    println!("The value of pop is: {:?}", pop);

    let two = nums[1];

    println!("The value of two is: {:?}", two);

    let one = nums.first();
    println!("The value of one is: {:?}", one);

    // .last
    // .first_mut and .last_mut to get mutable references to the first and last elements
    // len
    // is_empty

    nums.insert(1, 5);
    println!("The value of nums is: {:?}", nums);

    nums.remove(3);

    nums.sort();

    println!("The value of nums is: {:?}", nums);

    nums.reverse();
    println!("The value of nums is: {:?}", nums);

    nums.shuffle(&mut thread_rng());
    println!("The value of nums is: {:?}", nums);

    //===========================================================
    // BinaryHeap
    //===========================================================
    let mut bheap = BinaryHeap::new();

    bheap.push(1);
    bheap.push(18);
    bheap.push(20);
    bheap.push(5);

    println!("The value of bheap is: {:?}", bheap);

    bheap.pop();

    println!("The value of bheap is: {:?}", bheap.peek());

    println!("The value of bheap is: {:?}", bheap);

    //===========================================================
    // HashMap
    //===========================================================

    let mut map = HashMap::new();

    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    let old = map.insert("one", 4); // returns the old value

    println!("The value of map is: {:?}", map);

    let one = map.get("one");
    println!("The value of one is: {:?}", one);


    println!("{}", map.contains_key("one"));
    println!("{}", map.get("one").is_some());

    let keys:Vec<&str> = map.keys().cloned().collect();
    println!("The value of keys is: {:?}", keys);

    let values:Vec<&i32> = map.values().collect();
    println!("The value of values is: {:?}", values);


    //===========================================================
    // Sets
    //===========================================================

    let mut set = HashSet::new();

    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(1);

    for x in set.iter() {
        println!("The value of x is: {:?}", x);
    }

    println!("The value of set is: {:?}", set);

    let mut set2 = HashSet::new();

    set2.insert(1);
    set2.insert(2);
    set2.insert(12);
    set2.insert(13);


    for x in set.intersection(&set2) {
        println!("The intersection value of x is: {:?}", x);
    }

    for x in set.difference(&set2) {
        println!("The difference value of x is: {:?}", x);
    }

    for x in set.union(&set2) {
        println!("The union value of x is: {:?}", x);
    }
}
