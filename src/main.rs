use std::panic::{self, PanicInfo};

use mini_redis::{client, Result};

fn main() -> Result<()> {

    println!("before start");

    let some_var = std::panic::catch_unwind(|| {
        let a :PanicInfo = None.unwrap();
        println!("never runs");
    });

    println!("start");
    // let mut runtime = tokio::runtime::Runtime::new().unwrap();
    let mut runtime = tokio::runtime::Runtime::new();

    match runtime {
        Ok(mut runtime) => {
            runtime.block_on(async {
                let a = client::connect("127.0.0.1:6379");
            
                // Open a connection to the mini-redis address.
                let mut client = client::connect("127.0.0.1:6379").await?;
            
                // Set the key "hello" with value "world"
                client.set("hello", "world".into()).await? ;
            
                // Get key "hello"
                let result = client.get("hello").await?;
            
                println!("got value from the server; result={:?}", result);
            
                Ok(())
            })       
            
            }

            Err(e) => {
                println!("Error: {:?}", e);
                Ok(())
            }
    }  
}