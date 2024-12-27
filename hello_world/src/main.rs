use std::time::{SystemTime, UNIX_EPOCH};


#[tokio::main]
async fn main() {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("{}", time);
}

