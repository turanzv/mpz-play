use common::{tcp_connect, Role, DEFAULT_LOCAL};
use serio::{
    codec::{Bincode, Codec},
    stream::IoStreamExt,
    SinkExt,
};

#[tokio::main]
async fn main() {
    // Open a connection.
    println!("ALICE\tOpening connection with BOB...");
    let stream = tcp_connect(Role::Alice, DEFAULT_LOCAL).await.unwrap();
    let mut channel = Bincode.new_framed(stream);
    println!("ALICE\tConnection with BOB established.");


    // Send a number to Bob and wait for Bob's number.
    println!("ALICE\tSending number to BOB...");
    channel.send(12u32).await.unwrap();
    println!("ALICE\tSent 12 to BOB.");    

    // Print the number Alice received.
    let number: u32 = channel.expect_next().await.unwrap();
    println!("ALICE\tReceived from BOB: {}", number);
}
