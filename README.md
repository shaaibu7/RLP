# 🧬 Recursive Length Prefix (RLP) Encoder/Decoder in Rust

A blazing fast and lightweight implementation of the **Recursive Length Prefix (RLP)** encoding scheme in Rust. inspired by Ethereum's RLP specification used in the execution layer. This project provides essential RLP functionalities with a clean and modular design, and aims to be a go-to RLP utility for low-level data serialization and deserialization.

---

## ✨ Features

Currently implemented:

- ✅ Encode:
  - Empty string (`""`)
  - Boolean false (`false`)
  - Empty list (`[]`)
  - Short strings (`<= 55 bytes`)
  - Long strings (`>= 55 bytes`)
  - Single byte data (`0x00` to `0x7f`)
  
- ✅ Decode:
  - Single byte data
  - Short strings
  - Long strings

---

## 🛠️ Upcoming Features

In progress and planned for the next update:

- 🔜 Encode short and long lists (e.g. nested RLP structures)
- 🔜 Decode short and long lists
- 🔜 Fuzz testing and error handling improvements
- 🔜 Add high level abstraction for better development experience

---

## 🚀 Getting Started

### 📦 Prerequisites

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 📥 Clone the Repo


```bash
git clone https://github.com/shaaibu7/RLP.git
cd RLP
```