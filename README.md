# Boiled Water (`boilw`)

Eng | [中文](./README_CN.md)

**Boiled Water** (codenamed `boilw`) is the reference implementation of the **Bionic Intelligence (BI)** architecture.

BI is a deterministic, strong-logic intelligence system. It applies symbolist principles to mimic human thinking structures, building agents through rigorous logical mechanisms rather than probabilistic approximation.

## Core Concepts

`boilw` is strictly a **logical model**, fundamentally different from probabilistic models (such as LLMs).

- **Flag System**: The atomic unit of the system. All concepts, events, categories, and logic itself are deconstructed into Flags.
- **Topology as Logic**: Logic gates and reasoning pathways are expressed through the connection structure of the Flag graph, not through floating-point weights.
- **Memory Architecture**: Memory is structurally divided into "Experience" (Scene Events) and "Knowledge" (Flag Graph), supporting both temporary caching and permanent storage.

## Project Structure

This repository follows a monorepo structure.

```text
bi-boiled-water/
├── crates/
│   └── (Core components are currently under private development)
└── examples/
    ├── simple-flag/      # Demonstrates the fundamental Flag system and inference
    └── basic-logic/      # Demonstrates logical gates constructed via topology
```

> **Note**: The core runtime (`boilw-core`) and the standard plugins are currently under development and are not yet publicly released. The example projects provided in this repository contain simplified, inline implementations of the kernel to demonstrate the architectural principles.

## Usage

To run the provided examples, ensure you have the Rust toolchain installed.

```bash
# Clone the repository
git clone https://github.com/EyKettle/bi-boiled-water.git
cd bi-boiled-water

# Run the Simple Flag example
cargo run -p simple-flag

# Run the Basic Logic example
cargo run -p basic-logic
```

## License

The content currently available in this repository (Examples and Documentation) is released under the **Apache License, Version 2.0**.

See the [LICENSE](./LICENSE) file for details.

---

**Copyright © 2025 Boiled Water Project.**
