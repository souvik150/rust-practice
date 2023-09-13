use async_std::{fs::File, io, task, prelude::*};

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

fn main() {
    let task = task::spawn(async {
        let result = read_file("read.txt").await;
        match result {
            Ok(contents) => println!("contents: {}", contents),
            Err(e) => println!("error: {}", e),
        }
    });
    println!("Task spawned");

    task::block_on(task);
    println!("Task completed");
}


