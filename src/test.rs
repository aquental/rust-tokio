use std::thread;
use std::time;

use tokio::time::{sleep, Duration};
fn blocking_call() -> String {
    sleep(Duration::from_seconds(5)).await;
    "Finally done".to_string()
}

async fn async_call(ID: i32) -> String {
    sleep(Duration::from_seconds(1)).await;
    println!("Async call: ID {}", ID);
}

#[tokio::main]
async fn main() {
    let blocking_call_handle = tokio::task::spawn_blocking(blocking_call);
    let mut async_handles = Vec::new();
    for id in 0..15 {
        async_handles.push(tokio::task::spawn(async_call(id)));
    }
    for handle in async_handles {
        handle.await.unwrap();
    }
    let result = blocking_call_handle.await;
    println!("Result: {}", result.unwrap());
}
