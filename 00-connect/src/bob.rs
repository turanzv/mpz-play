use common::{tcp_connect, Role, DEFAULT_LOCAL};
use serio::{
    codec::{Bincode, Codec},
    stream::IoStreamExt,
    SinkExt,
};

#[tokio::main]
async fn main() {
    // Open a connection.
    println!("BOB\tOpening connection with ALICE...");
    let stream = tcp_connect(Role::Bob, DEFAULT_LOCAL).await.unwrap();
    let mut channel = Bincode.new_framed(stream);
    println!("BOB\tConnection with ALICE established.");

    // Wait for Alice to send her number and print it out.
    println!("BOB\tWaiting for Alice's number...");
    let number:u32 = channel.expect_next().await.unwrap();
    println!("BOB\tAlice sent: {}", number);

    // Increment and send it back.
    println!("BOB\tSending back incremented number...");
    channel.send(number + 1).await.unwrap();
    println!("BOB\tSent back incremented number.");

}
