# Burn Backend Selection Plan

To handle both CPU and GPU environments efficiently in Rust using the `burn` library, we will use **Cargo Features**. This allows the user to compile the binary with the appropriate backend for their hardware.

## 1. Cargo.toml Configuration

We will define features that map to `burn` backends:
- `wgpu`: For GPU acceleration (cross-platform).
- `ndarray`: For CPU-based execution (default).

```toml
[package]
name = "neat-hop"
version = "0.1.0"
edition = "2021"

[dependencies]
burn = { version = "0.13", features = ["train"] }
serde = { version = "1.0", features = ["derive"] }

# Backend specific dependencies
burn-ndarray = { version = "0.13", optional = true }
burn-wgpu = { version = "0.13", optional = true }

[features]
default = ["ndarray"]
ndarray = ["dep:burn-ndarray"]
wgpu = ["dep:burn-wgpu"]
```

## 2. Code Implementation (main.rs)

We will use conditional compilation (`#[cfg(feature = "...")]`) to provide a unified interface for the backend.

```rust
#[cfg(feature = "wgpu")]
type Backend = burn_wgpu::WgpuBackend<burn_wgpu::AutoGraphicsApi, f32, i32>;

#[cfg(all(feature = "ndarray", not(feature = "wgpu")))]
type Backend = burn_ndarray::NdArrayBackend<f32>;

fn main() {
    println!("Hello, world!");
    
    #[cfg(feature = "wgpu")]
    println!("Running with WGPU (GPU) backend");
    
    #[cfg(all(feature = "ndarray", not(feature = "wgpu")))]
    println!("Running with NdArray (CPU) backend");
}
```

## 3. Usage

- **Default (CPU):** `cargo run`
- **GPU:** `cargo run --features wgpu --no-default-features`

This approach ensures that the binary only includes the necessary dependencies for the target hardware, keeping the build lean and avoiding compilation errors on systems without GPU drivers if only CPU is needed.
