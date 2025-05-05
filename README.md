# Tide

High-performance data streaming for Solana validators. Tide provides ultra-low latency blockchain data streaming by directly tapping into Agave validator internals, bypassing traditional Geyser limitations.

## Overview

Tide achieves sub-millisecond latency by:

- Memory-mapped direct access to validator TPU/TVU pipelines
- SIMD-optimized data processing
- Zero-copy serialization
- Efficient output formats (JSON5, Parquet)

## Architecture

```
Validator TPU → MMAP → Tide Core → SIMD Processing → Formatter → Output
```

## Key Features

- **Ultra-low latency**: aiming 10ms median latency
- **Efficient formats**: JSON5 for debugging, Parquet for production
- **Direct validator integration**: Works with Agave/Firedancer
- **Minimal dependencies**: Focused on performance
- **Configurable output**: Flexible output format selection

## Quick Start

### Prerequisites

- Rust 1.81+
- Access to a Solana validator
- Linux system (for optimal performance)

### Building

```bash
cargo build --release
```

### Configuration

Create a configuration file `config.toml`:

```toml
[output]
format = "parquet"  # or "json5"
path = "/data/tide-output"

[performance]
pipeline_stages = 4
batch_size = 1024
```

### Running with Agave

```bash
# As a geyser plugin
solana-test-validator --geyser-plugin-config config.toml

# Standalone mode
tide-cli --config config.toml
```

## Benchmarks

```bash
cargo bench
```

## License

MIT OR Apache-2.0

## Contributing

Contributions welcome! Please read CONTRIBUTING.md first.

## Security

Please report security issues responsibly to vivek@windnetowrk.ai or post issue on this GitHub repo.