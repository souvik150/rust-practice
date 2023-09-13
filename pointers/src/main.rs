// use std::rc::Rc;
// use std::sync::Arc;
// use std::cell::RefCell;

// stuct Flagger {
//     is_true: RefCell<bool>,
// }

// struct Flagger {
//     is_true: bool,
// }

// interior mutability
// mutability is enforced at runtime

// RefCell<T> is used when we want to mutate data but we don't want to use



fn main() {
    // let t = (12, "eggs");
    // let b = Box::new(t); //created on heap but b is stored on stack
    // println!("{:?}", b);

    // let x = 5;
    // let y = &x;

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let z = Box::new(x);

    // assert_eq!(5, *z);

    // =====================================================
    // =================  RC and Arc =======================
    // =====================================================

    // Rc - reference counting
    // Arc - atomic reference counting

    // Rc is not thread safe
    // Arc is thread safe

    // Rc is used when we want to share data between multiple parts of our program
    // but we know that there will be a single owner at any given time

    // Arc is used when we want to share data between multiple parts of our program
    // and we don't know how many owners there will be at any given time

    // let s1 = Rc::new(String::from("Hello"));
    // let s2 = s1.clone();
    // let s3 = s2.clone();

    // reference count is 3
    // println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3);

    // =====================================================
    // =================  RefCell ==========================
    // =====================================================

    // let flag = Flagger { is_true: RefCell::new(true) }; //borrow and borrow mut

    // let mut mut_ref = flag.is_true.borrow_mut();
    // *mut_ref = false;
    // println!("flag = {:?}", mut_ref);

    //borrow returns Ref<T>
    //borrow_mut returns RefMut<T>

    // RefCell is used when we want to mutate data but we don't want to use
}
