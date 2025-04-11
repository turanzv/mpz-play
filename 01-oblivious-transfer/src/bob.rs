use common::{tcp_connect, Role, DEFAULT_LOCAL};
use serio::codec::{Bincode, Codec};
use mpz_common::executor::STExecutor;
use mpz_ot::{
    chou_orlandi::{Receiver, ReceiverConfig},
    OTReceiver, OTSetup,
};

#[tokio::main]
async fn main() {
    // Open a connection and create a channel.
    let stream = tcp_connect(Role::Bob, DEFAULT_LOCAL).await.unwrap();
    let channel = Bincode.new_framed(stream);
    
    // Create an executor.
    let mut executor = STExecutor::new(channel);

    // Create an OT receiver and set it up.
    let receiver_config = ReceiverConfig::default();
    let mut receiver = Receiver::new(receiver_config);

    println!("BOB\tSetting up receiver...");
    receiver.setup(&mut executor).await.unwrap();
    println!("BOB\tReceiver setup.");

    // Make a choice.
    let choice = true;
    println!("BOB\tChoice: {}", choice);

    // Receive OTs from Alice.
    let msg = receiver.receive(&mut executor, &[choice]).await.unwrap();
    println!("BOB\tReceived from Alice: {:?}", msg.msgs.first().unwrap());

}
