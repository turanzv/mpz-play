## 📘 Exercise Summary: OT - Garbled Circuits

### 🧠 Goal

Securely evaluate an AES-128 encryption circuit using **garbled circuits**, where:
- **Alice** provides the encryption key as a *blind input*.
- **Bob** provides the plaintext message as a *private input*.
- The AES encryption is performed securely, and only Alice receives the final ciphertext.

This uses:
- Oblivious transfer for input label distribution,
- A garbled circuit virtual machine (VM) for evaluation,
- Input and output wire labeling for secure computation.

---

### ℹ️ Background Concepts

#### 🧱 What Are Garbled Circuits?

**Garbled circuits** are a cryptographic protocol for secure two-party computation. One party (the *garbler*) encrypts a Boolean circuit such that another party (the *evaluator*) can compute it without learning any internal values.

- **Input Labels**: Each wire in the circuit has two labels, one for logical 0 and one for logical 1. Only one label per input is revealed to the evaluator.
- **Output Labels**: The garbler maps the final output wire labels back to plaintext results.
- **VM Abstraction**: The `garble_vm` provides a high-level interface to define inputs, assign values, execute the circuit, and decode outputs.

---

### 🧩 Data Types and Their Usage

| Type                                | Purpose                                                                  |
|-------------------------------------|--------------------------------------------------------------------------|
| `AES128`                            | Predefined AES-128 ECB-mode Boolean circuit.                             |
| `STExecutor`                        | Asynchronous execution context over a TCP channel.                       |
| `Memory`                            | Manages inputs, outputs, and assignments in the garbled circuit VM.      |
| `Execute`, `DecodePrivate`          | Traits for circuit execution and label decoding.                         |
| `new_private_input` / `new_blind_input` | Declare party-specific and blind (OT-based) inputs.                      |
| `assign()`                          | Sets a known value for a private input wire.                             |

---

### 🛠️ Key Methods

| Method                                 | Description                                                               |
|----------------------------------------|---------------------------------------------------------------------------|
| `setup_garble(role, executor, size)`   | Initializes the garbled circuit VM with the given role.                   |
| `vm.new_private_input::<T>(name)`      | Defines a private input for the local party.                              |
| `vm.new_blind_input::<T>(name)`        | Defines a blind (OT-based) input for the remote party.                    |
| `vm.new_output::<T>(name)`             | Declares an output wire to decode after execution.                        |
| `vm.assign(&input, value)`             | Assigns a known value to a local private input.                           |
| `vm.execute(circuit, inputs, outputs)` | Evaluates the Boolean circuit using provided inputs and tracks outputs.   |
| `vm.decode_blind(&[outputs])`          | Sends decoding information for blind outputs back to the garbler.         |

---

### 🔧 Execution Flow

1. **Connection & Executor Setup**:
   - Each party uses `tcp_connect()` and `STExecutor::new()` to establish a framed channel.

2. **VM Initialization**:
   - Garbled circuit VM is instantiated using `setup_garble(...)`.

3. **Input Wiring**:
   - Alice creates a **blind input** for the AES key.
   - Bob creates a **private input** and **assigns** the plaintext message.
   - Both declare a shared **output wire** for the ciphertext.

4. **Circuit Execution**:
   - The `AES128` circuit is cloned and executed via `vm.execute(...)`.

5. **Result Decoding**:
   - Alice calls `decode_blind(...)` to obtain the actual ciphertext from output labels.

---

### 🔍 Highlights

- Garbled circuits allow Bob to evaluate Alice’s encrypted logic without learning her key.
- **Input/output labels** enforce security: Bob never sees full wire mappings, only his selected labels.
- OT is used internally to transfer blind input labels (Alice’s key) to Bob without revealing the key bits.
- The VM interface abstracts low-level wire logic and allows for high-level, role-safe interaction.

#### ✨ `.clone()` Usage

| Line                            | Type Being Cloned         | What `.clone()` Does                       | Why It's Needed                             |
|---------------------------------|---------------------------|---------------------------------------------|---------------------------------------------|
| `let circuit = AES128.clone();` | `Arc<Circuit>`            | Clones a reference-counted circuit handle   | Needed because `execute()` takes ownership  |
| `ciphertext.clone()`            | Output handle (label ref) | Duplicates a reference to the output label  | Used in both `execute()` and `decode_blind()`|

- `AES128` is wrapped in an `Arc`, so `clone()` is cheap (just bumps ref count).
- `ciphertext.clone()` preserves the original output handle across stages in the protocol.

---

### 📝 Notes for Future Protocols

- ✅ Garbled circuit VMs abstract away wire-level operations for safer logic composition.
- ✅ Blind inputs automatically integrate OT-based label selection.
- ⚠️ Every wire (input/output) must be declared before execution and decoded appropriately.
- 📌 Secure circuit evaluation via garbled circuits is foundational to general secure computation.