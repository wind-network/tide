# Tide Architecture

## Overview

Tide is a high-performance data streaming solution designed to extract data directly from Solana validator internals, bypassing traditional bottlenecks associated with the Geyser plugin interface.

## Core Components

### 1. Validator Integration Layer

The validator integration layer implements direct memory-mapped access to the validator's internal pipelines:

- **TPU Pipeline Tap**: Direct access to Transaction Processing Unit data
- **TVU Pipeline Tap**: Direct access to Transaction Verification Unit data
- **MMAP Interface**: Memory-mapped shared buffers for ultra-low latency
- **SIMD Processing**: AVX-512 optimized data processing

### 2. Streaming Engine

The streaming engine consists of:

- **Pipeline Processor**: Multi-stage async processing pipeline
- **Data Transformation**: Zero-copy serialization and deserialization
- **Batch Processor**: Intelligent batching for optimal throughput
- **Output Formatter**: Support for JSON5 and Parquet formats

### 3. Configuration System

Flexible configuration supporting:
- Output format selection
- Performance tuning parameters
- Validator connection settings
- Rotation and compression policies

## Data Flow

```
Agave Validator TPU → MMAP Buffer → SIMD Processor → 
Stream Pipeline → Batch Aggregator → Formatter → Output
```

## Performance Optimizations

### Memory Management
- Cache-aligned data structures
- Zero-copy data paths where possible
- Memory pool allocation for high-frequency operations

### Processing Optimizations
- SIMD (AVX-512) vectorized operations
- Parallel processing pipelines
- Minimal locking with lock-free queues

### I/O Optimizations
- Direct I/O for file operations
- Asynchronous writing
- Intelligent buffering strategies

## Integration Points

### Agave Integration
Tide integrates with Agave validators through:
1. Direct memory access to internal pipelines
2. Geyser plugin interface (fallback)
3. RPC interface (debugging)

### Output Formats

#### JSON5
- Human-readable format
- Ideal for debugging and testing
- Slightly larger file size

#### Parquet
- Production-optimized format
- Columnar storage for analytics
- Significant compression benefits

## Scalability Considerations

Tide is designed to scale with:
- Multiple worker threads for parallel processing
- Dynamic batch sizing based on load
- Adaptive memory allocation
- Configurable pipeline stages

## Security Model

- Minimal external dependencies
- Isolated processing components
- Memory-safe Rust implementation
- No network exposure required