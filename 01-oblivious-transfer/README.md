## 📘 Exercise Summary: 01 - Oblivious Transfer (OT)

### 🧠 Goal

Implement a basic 1-out-of-2 Oblivious Transfer (OT) using the Chou-Orlandi protocol, where:
- **Alice** (the sender) defines two messages.
- **Bob** (the receiver) selects one using a choice bit, without Alice learning which.
- Bob learns only the selected message, and Alice learns nothing about Bob’s choice.

---

### 🧩 Data Types and Their Usage

| Type                             | Purpose                                                                 |
|----------------------------------|-------------------------------------------------------------------------|
| `Block`                          | Represents a 128-bit message (used for OT message payloads).           |
| `Role`                           | Determines party role (Alice = sender, Bob = receiver).                |
| `Bincode`                        | Codec for serializing/deserializing over the TCP channel.              |
| `Framed<TcpStream, Bincode>`     | Used for structured async message passing.                             |
| `STExecutor`                     | Provides a communication context for executing the protocol.           |
| `SenderConfig`, `ReceiverConfig`| Configuration structs for Chou-Orlandi OT parties.                     |
| `OTSender`, `OTReceiver`         | Trait interfaces for sending/receiving in the OT protocol.             |

---

### 🛠️ Key Methods

| Method                                     | Description                                                                 |
|--------------------------------------------|-----------------------------------------------------------------------------|
| `tcp_connect(Role, addr).await`            | Establishes a TCP connection between the sender and receiver.              |
| `Bincode.new_framed(stream)`               | Wraps the TCP connection into a framed, typed message stream.              |
| `STExecutor::new(channel)`                 | Wraps the channel into a protocol execution context.                        |
| `OTSetup::setup(executor)`         | Initializes sender or receiver from a config and executor.                 |
| `sender.send(inputs).await`                | Sends two messages per OT selection (indexed by 0 and 1).                  |
| `receiver.receive(&[choice_bit]).await`    | Retrieves the message corresponding to the receiver’s choice.             |

---

### 🔧 Setting Up OT Sender and Receiver

#### 🔐 Alice (OT Sender)
```rust
let config = SenderConfig::default();
let sender = Sender::setup(executor).await.unwrap();
let inputs = vec![
    [Block::from(0_u128), Block::from(1_u128)], // pair of messages
];
sender.send(&inputs).await.unwrap();
```
- Uses SenderConfig::default() to configure the sender.
- Prepares a vector of message pairs (Block type).
- Calls .send() to engage in oblivious transfer with the receiver.

#### 🔑 Bob (OT Receiver)
```Rust
let config = ReceiverConfig::default();
let receiver = Receiver::setup(executor).await.unwrap();
let choices = vec![true]; // Bob chooses the second message (index 1)
let outputs = receiver.receive(&choices).await.unwrap();
```
Uses ReceiverConfig::default() to configure the receiver.
Specifies choice bits (true = index 1, false = index 0).
Calls .receive() to retrieve only the selected message.

### 🔍 Hihglights
- Alice defines 2 messages per OT instance; Bob selects one without revealing his choice.
- The communication is wrapped in an STExecutor to abstract over async, framed message flow.
- Privacy is preserved: the sender never learns Bob’s choice, and Bob never learns the unchosen message.
- This pattern is foundational for building garbled circuits and secure two-party computations.
- 🧭 The `Sender` and `Receiver` setup uses the **typestate pattern**:
  - Configuration (`SenderConfig` / `ReceiverConfig`) and setup (`setup(...)`) are separate from usage (`send()` / `receive()`).
  - This enforces protocol correctness at the type level — you can’t send or receive until the party is properly initialized.
  - Helps prevent misuse of the protocol state machine at compile time.

### 📝 Notes for Future Protocols
- ✅ Use Block to represent fixed-size secure messages efficiently.
- ✅ Chou-Orlandi OT is efficient and compatible with many higher-level protocols.
- ⚠️ Ensure both sides agree on the number of OTs and the shape of the input vectors.
- 📌 OT enables private input transfer — critical in secure multi-party computation.