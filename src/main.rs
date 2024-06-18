use std::thread::sleep;


fn main() {
    let mut i = 0;
    while i < 6 {
        println!("Hello, world!");
        sleep(std::time::Duration::from_secs(1));
        i += 1;
    }
}

