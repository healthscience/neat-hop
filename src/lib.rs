#[cfg(feature = "wgpu-backend")]
pub type Backend = burn::backend::Wgpu<f32, i32>;

#[cfg(all(feature = "ndarray-backend", not(feature = "wgpu-backend")))]
pub type Backend = burn::backend::NdArray<f32>;

pub fn run() {
    println!("Hello, world!");
    
    #[cfg(feature = "wgpu-backend")]
    println!("Running with WGPU (GPU) backend");
    
    #[cfg(all(feature = "ndarray-backend", not(feature = "wgpu-backend")))]
    println!("Running with NdArray (CPU) backend");
}
