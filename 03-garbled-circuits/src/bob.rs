use common::{tcp_connect, Role, DEFAULT_LOCAL};
use garbled_circuits::setup_garble;
use mpz_common::executor::STExecutor;
use serio::codec::{Bincode, Codec};

#[tokio::main]
async fn main() {
    // Open a connection.
    let tcp = tcp_connect(Role::Bob, DEFAULT_LOCAL).await.unwrap();
    let channel = Bincode.new_framed(tcp);

    // Create an executor and use it to instantiate a vm for garbled circuits.
    let executor = STExecutor::new(channel);
    let mut _garble_vm = setup_garble(Role::Bob, executor, 256).await.unwrap();

    // Define input and output types.

    // Assign the message.

    // Load the AES circuit.

    // Execute the circuit.

    // Send output information to Alice.
}
