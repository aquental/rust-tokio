use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Sleep};

async fn person(remote: Arc<Mutex<i32>>, name: String, new_channel: i32) {
    //request access to remote
    let mut real_remote = remote.lock().await;

    //change tv channel
    *real_remote = new_channel;
    println!("{} changed tv channel to {}", name, new_channel);

    sleep::Duration::from_secs(10).await;
}

#[tokio::main]
async fn main() {
    let tv_channel = 10;
    let remote = Mutex::new(tv_channel);
    let remote_arc = Arc::new(remote);

    let mut task_handles = Vec::new();
    for (name, channel) in [
        ("Alice", 11),
        ("Bob", 12),
        ("Carol", 13),
        ("Dana", 14),
        ("Eve", 15),
    ] {
        task_handles.push(tokio::spawn(person(
            remote_arc.clone(),
            name.to_string(),
            channel,
        )));

        for handle in task_handles {
            handle.await.unwrap();
        }
    }
}
