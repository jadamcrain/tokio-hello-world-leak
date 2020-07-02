

/*
#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
*/

fn main() {
    let mut runtime = tokio::runtime::Runtime::new().unwrap();

    let handle = runtime.spawn(async move {
        println!("hello world!");
    });

    runtime.block_on(handle).unwrap();
}
