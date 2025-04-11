use common::{tcp_connect, Role, DEFAULT_LOCAL};
use serio::codec::{Bincode, Codec};
use mpz_common::executor::STExecutor;
use mpz_core::Block;
use mpz_ot::{
    chou_orlandi::{Sender, SenderConfig},
    OTSender, OTSetup,
};

#[tokio::main]
async fn main() {
    // Open a connection and create a channel.
    let stream = tcp_connect(Role::Alice, DEFAULT_LOCAL).await.unwrap();
    let channel = Bincode.new_framed(stream);

    // Create an executor.
    let mut executor = STExecutor::new(channel);
    
    // Create an OT sender and set it up.
    let sender_config = SenderConfig::default();
    let mut sender = Sender::new(sender_config);

    println!("ALICE\tSetting up sender...");
    sender.setup(&mut executor).await.unwrap();
    println!("ALICE\tSender setup.");

    // Create messages.
    let zero = Block::ZERO;
    let one = Block::ONE;
    println!("ALICE\tFalse: {:?}", zero);
    println!("ALICE\tTrue:  {:?}", one);

    // Send OTs to Bob.
    sender.send(&mut executor, &[[zero, one]]).await.unwrap();
}
