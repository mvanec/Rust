use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::io;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

pub struct MyFuture {
    pub state: i32,
}

impl Future for MyFuture {
    type Output = i32;
    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if self.state == 0 {
            self.state += 1;
            println!("Zero State = {}", self.state);
            Poll::Pending
        } else {
            println!("State = {}", self.state);
            Poll::Ready(self.state)
        }
    }
}
