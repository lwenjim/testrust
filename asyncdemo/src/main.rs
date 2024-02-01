use std::future::Future;

fn future_print() -> impl Future<Output = ()> {
    async {
        println!("are you ok");
    }
}
async fn future_print2() {
    println!("are you ok");
}
fn main() {
    future_print();
    future_print2();
}

