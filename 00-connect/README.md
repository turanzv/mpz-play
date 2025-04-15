## 📘 Exercise Summary: Basic Channel Communication

### 🧠 Goal

Establish a bidirectional connection between two parties (Alice and Bob), and exchange simple structured data using a framed I/O channel with automatic serialization.

---

### 🧩 Data Types and Their Usage

| Type                            | Purpose                                                                 |
|---------------------------------|-------------------------------------------------------------------------|
| `Role`                          | Enum (`Role::Alice`, `Role::Bob`) to determine connection direction.   |
| `Bincode`                       | Codec for serializing and deserializing messages over the TCP channel. |
| `Framed<TcpStream, Bincode>`    | Wraps the TCP stream to send/receive typed messages asynchronously.    |
| `IoStreamExt`, `SinkExt`        | Trait extensions providing `.send()` and `.next().await` on framed I/O.|
| `u32`                           | Data payload sent between Alice and Bob. Used as a simple test message.|

---

### 🛠️ Key Methods

| Method                          | Description                                                                 |
|---------------------------------|-----------------------------------------------------------------------------|
| `tcp_connect(Role, addr).await` | Establishes a TCP connection as either Alice or Bob.                       |
| `Bincode.new_framed(stream)`    | Wraps a TCP stream in a `Framed` object using the `Bincode` codec.         |
| `channel.send(value).await`     | Sends a serializable value over the channel.                               |
| `channel.expect_next().await`          | Receives the next message from the channel as an `Option<Result<T>>`.      |

---

### 🔍 Highlights

- The protocol begins with each party using `tcp_connect()` to initiate or wait for a connection.
- The raw TCP stream is wrapped with `Bincode.new_framed()` to enable high-level message I/O.
- Alice sends a `u32` value to Bob using `.send().await`.
- Bob receives Alice’s message using `.next().await`, processes it, and sends a reply.
- The framed channel handles both serialization and deserialization automatically.
- This pattern forms the basic foundation for future secure computation protocols.

---

### 📝 Notes for Future Protocols

- ✅ Reusable pattern for any client-server or peer-to-peer message-passing setup.
- ⚠️ Ensure both sides follow the same send/receive order to prevent deadlocks.
- 📌 `serio` + `tokio` + `Framed` simplifies low-level networking into safe, typed async I/O.