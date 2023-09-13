use std::fs::File;
use std::io::ErrorKind;
use std::fs::rename;
use std::io::Error;

fn main() {
    //panic!("panic here")

    let file = File::open("error.txt");

    // match

    // let file = match file {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("error.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
    //         }
    //     },

    //     _  => panic!("There was some other error"),
    // };


    // ========================================================
    // unwrap
    // ========================================================

    // let file = File::open("error.txt").unwrap();

    // ========================================================
    // expect
    // ========================================================

    // let file = File::open("error.txt").expect("Failed to open error.txt");


    // ========================================================
    // Propagating Errors
    // ========================================================

    let test = open_file();
    test.unwrap();

}

fn open_file() -> Result<File, Error> {
    let file = File::open("error.txt")?;
    Ok(file)
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
