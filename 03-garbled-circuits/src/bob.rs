use common::{tcp_connect, Role, DEFAULT_LOCAL};
use garbled_circuits::setup_garble;
use mpz_circuits::circuits::AES128;
use mpz_common::executor::STExecutor;
use mpz_garble::{DecodePrivate, Execute, Memory};
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
    let key = _garble_vm.new_blind_input::<[u8; 16]>("key").unwrap();
    let msg = _garble_vm.new_private_input::<[u8; 16]>("msg").unwrap();
    let ciphertext = _garble_vm.new_output::<[u8; 16]>("ciphertext").unwrap();

    // Assign the message.
    _garble_vm.assign(
        &msg,
        [
            0x6b_u8, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96, 0xe9, 0x3d, 0x7e, 0x11, 0x73,
            0x93, 0x17, 0x2a,
        ]
    ).unwrap();

    // Load the AES circuit.
    let circuit = AES128.clone();
    // AES128 is a shared instance: `pub static AES128: circuit`
    // clone() is needed here to make a local owned copy of the circuit object.
    // a shallow copy: ERS128 is an Arc<Circuit>

    // Execute the circuit.
    _garble_vm
        .execute(circuit, &[key, msg], &[ciphertext.clone()])
        // clone is used here to pass an instance of ciphertext that .execute() owns
        // the local ciphertext is kept here to be used on line 47
        .await
        .unwrap();

    // Send output information to Alice.
    _garble_vm.decode_blind(&[ciphertext]).await.unwrap();
}
