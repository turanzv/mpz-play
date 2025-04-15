## 📘 Exercise Summary: 02 - Finite Fields and Share Conversion

### 🧠 Goal

Demonstrate the use of **share conversion techniques** in finite fields using `mpz`. The exercise shows how to:
- Convert a shared additive value into a shared multiplicative value.
- Then convert it back to an additive form.
- Verify correctness by comparing the initial and final additive shares.

This involves:
- OLE-based oblivious transfer.
- Additive and multiplicative share representations.
- Cross-party consistency checks via message exchange.

---

### ℹ️ Background Concepts

#### 🔁 What is OLE?

**Oblivious Linear Evaluation (OLE)** is a cryptographic primitive that lets one party (the sender) provide a function of the form `f(x) = ax + b` without revealing `a` or `b`, and the other party (the receiver) learn `f(x)` for their input `x`, without revealing `x`. It's used here as the secure mechanism for performing arithmetic over secret shares.

#### ➕ vs ✖️ Additive and Multiplicative Shares

- **Additive Shares**: A value `v` is split into `v = a + b`, where `a` and `b` are random shares known to each party. Addition of shared values is easy.
- **Multiplicative Shares**: A value `v` is represented as `v = a * b`, which supports efficient multiplication of shared secrets.

🔄 **Conversion is necessary** because different computations (like sums and products) are more efficient in different representations. Secure protocols often switch between these forms to optimize performance and security.

---

### 🧩 Data Types and Their Usage

| Type                                           | Purpose                                                                 |
|------------------------------------------------|-------------------------------------------------------------------------|
| `OLEReceiver`, `OLESender`                     | Provide Oblivious Linear Evaluation capability for finite field ops.   |
| `ShareConversionReceiver`, `ShareConversionSender` | Abstract the conversion of shares between additive and multiplicative. |
| `AdditiveShare<T>` / `MultiplicativeShare<T>` | Represent shared values in different algebraic forms.                  |
| `AdditiveToMultiplicative`                    | Performs conversion from additive to multiplicative shares.            |
| `MultiplicativeToAdditive`                    | Converts back to additive shares.                                      |
| `FieldElement`                                | Represents an element in the finite field used for computation.        |

---

### 🛠️ Key Methods

| Method                                                      | Description                                                                 |
|-------------------------------------------------------------|-----------------------------------------------------------------------------|
| `OLE{Sender,Receiver}::setup(...)`                          | Initializes the base OLE provider using Chou-Orlandi OT.                   |
| `ShareConversion{Sender,Receiver}::new(...)`                | Wraps an OLE provider into a share conversion object.                      |
| `AdditiveToMultiplicative::convert(...)`                    | Converts additive shares to multiplicative form securely.                  |
| `MultiplicativeToAdditive::convert(...)`                    | Reverses the transformation, returning to additive form.                   |
| `send()` / `receive()` (final check)                        | Parties exchange final summands to verify correctness of share recovery.  |

---

### 🔧 Setup & Share Conversion Flow

#### 1. Initialize OLE (Oblivious Linear Evaluation)
```rust
let base = OLESender::setup(BaseSenderConfig::default(), executor).await.unwrap();
let conv = ShareConversionSender::new(base);
```
- Uses a Chou-Orlandi-based sender or receiver as the base.
- Wraps it to enable share conversion routines.

#### 2. Convert Additive → Multiplicative
```rust
let (share_sender, share_receiver) = AdditiveToMultiplicative::convert(&conv, share).await.unwrap();
```
- Securely transforms the shared value into a multiplicative format using the OLE backend.

#### 3. Convert Multiplicative → Additive
```rust
let (new_share_sender, new_share_receiver) = MultiplicativeToAdditive::convert(&conv, share).await.unwrap();
```

#### 4. Validate Equality
- Each party sends the missing share piece to the other.
- Locally reconstruct and compare to confirm the round-trip transformation succeeded.

### 🔍 Highlights
- Demonstrates key concepts in secure computation over finite fields:
 - Additive and multiplicative secret sharing.
 - Use of OLE for algebraic transformations without leaking values.
- Share conversion is fundamental for switching between arithmetic representations during secure computation.
- The conversion process is done securely using OLE, not through plain value exchange.

### 📝 Notes for Future Protocols
- ✅ Use OLE as a foundation for efficient arithmetic secret-sharing operations.
- ✅ ShareConversion types encapsulate the logic for secure algebraic transformations.
- ⚠️ Always verify correctness via recombination of shares after transformation.
- 📌 This pattern is crucial for protocols that alternate between additive and multiplicative operations, such as secure neural networks or multi-party arithmetic circuits.
