// use futures::executor;

async fn hello_world() {
futures_timer::Delay::new(std::time::Duration::from_secs(3)).await;
    println!("hello, world!");
}

fn main() {
    println!("start");

    futures::executor::block_on(hello_world());
    println!("end");

}