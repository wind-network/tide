<div align="center">

# 🌊 Tide

<img src="https://github.com/user-attachments/assets/a6f0d22c-7f55-4f9b-81d6-1a9b24a2e334" alt="Tide Logo" width="80" height="80">

**High-performance data streaming for Solana validators**

*Ultra-low latency blockchain data streaming by Wind Network*

[![Build Status](https://img.shields.io/github/actions/workflow/status/wind-network/tide/ci.yml?branch=main&style=for-the-badge)](https://github.com/wind-network/tide/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=for-the-badge)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.81+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![Solana](https://img.shields.io/badge/solana-compatible-purple?style=for-the-badge&logo=solana)](https://solana.com)

[Quick Start](#-quick-start) • [Documentation](#-documentation) • [Benchmarks](#-benchmarks) • [Contributing](#-contributing)

---

</div>

## 🚀 Overview

<div align="center">

**Tide revolutionizes Solana data streaming by bypassing traditional Geyser limitations**

</div>

Tide achieves **sub-millisecond latency** through innovative architecture:

<div align="center">

| Feature | Technology | Benefit |
|---------|------------|---------|
| 🔗 **Direct Access** | Memory-mapped TPU/TVU pipelines | Zero intermediary overhead |
| ⚡ **SIMD Processing** | Vectorized data operations | Maximum CPU efficiency |
| 📦 **Zero-Copy** | In-place serialization | Minimal memory allocation |
| 📊 **Flexible Output** | JSON5 & Parquet formats | Debug-friendly & production-ready |

</div>

</div>

## ✨ Key Features

<div align="center">

### 🎯 **Performance First**
Targeting **100ms median latency** with sub-millisecond peaks

### 🔧 **Production Ready**
JSON5 for debugging • Parquet for production • Configurable everything

### 🔌 **Native Integration**
Works seamlessly with **Agave** and **Firedancer** validators

### 📦 **Minimal Footprint**
Focused dependencies • Maximum performance per resource

</div>

---

## 🚀 Quick Start

<div align="center">

### Prerequisites

![Rust](https://img.shields.io/badge/Rust-1.81+-orange?style=flat-square&logo=rust)
![Linux](https://img.shields.io/badge/Linux-Required-blue?style=flat-square&logo=linux)
![Solana](https://img.shields.io/badge/Solana-Validator-purple?style=flat-square&logo=solana)

</div>

### 📦 Installation

```bash
# Clone the repository
git clone https://github.com/your-org/tide.git
cd tide

# Build optimized release
cargo build --release
```

### ⚙️ Configuration

Create your `config.toml`:

```toml
[output]
format = "parquet"  # Options: "parquet" | "json5"
path = "/data/tide-output"
compression = "zstd"

[performance]
pipeline_stages = 4
batch_size = 1024
enable_simd = true

[validator]
tpu_address = "127.0.0.1:1024"
connection_timeout = "5s"
```

### 🏃‍♂️ Running

<div align="center">

**As Geyser Plugin**
```bash
solana-test-validator --geyser-plugin-config config.toml
```

**Standalone Mode**
```bash
tide-cli --config config.toml --verbose
```

</div>

---

## 📊 Benchmarks

<div align="center">

</div>

Run benchmarks locally:

```bash
cargo bench --features benchmark
```

View detailed results:
```bash
open target/criterion/report/index.html
```

---

## 📚 Documentation

<div align="center">

| Resource | Description |
|----------|-------------|
| [📖 **API Docs**](https://docs.rs/tide) | Complete API reference |
| [🔧 **Configuration Guide**](docs/config.md) | Detailed setup instructions |
| [🏗️ **Architecture**](docs/architecture.md) | System design deep-dive |
| [🔬 **Performance Tuning**](docs/performance.md) | Optimization guidelines |

</div>

---

## 🤝 Contributing

<div align="center">

**We welcome contributions from the community!**

[![Contributors](https://img.shields.io/github/contributors/your-org/tide?style=for-the-badge)](https://github.com/your-org/tide/graphs/contributors)
[![Issues](https://img.shields.io/github/issues/your-org/tide?style=for-the-badge)](https://github.com/your-org/tide/issues)
[![PRs](https://img.shields.io/github/issues-pr/your-org/tide?style=for-the-badge)](https://github.com/your-org/tide/pulls)

</div>

### 📋 Getting Started

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

Please read our [Contributing Guidelines](CONTRIBUTING.md) before submitting PRs.

---

## 🔒 Security

<div align="center">

**Security is our top priority**

Found a vulnerability? Please report it responsibly:

📧 **Email**: [vivek@windnetwork.ai](mailto:vivek@windnetwork.ai)  
🐛 **GitHub**: [Create Security Issue](https://github.com/your-org/tide/security/advisories/new)

</div>

---

## 📄 License

<div align="center">

This project is licensed under **MIT OR Apache-2.0**

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](https://choosealicense.com/licenses/mit/)
[![Apache License](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=for-the-badge)](https://choosealicense.com/licenses/apache-2.0/)

</div>

---

<div align="center">

**Made with ❤️ for the Solana ecosystem**

⭐ **Star us on GitHub** if this project helped you!

[⬆ Back to Top](#-tide)

</div>
