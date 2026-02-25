use neat_hop::Backend;
use burn::tensor::Tensor;

#[test]
fn test_backend_tensor_creation() {
    // This test verifies that we can create a tensor using the selected Backend.
    // It will run on whichever backend is currently enabled via features.
    let device = Default::default();
    let tensor = Tensor::<Backend, 1>::from_floats([1.0, 2.0, 3.0], &device);
    assert_eq!(tensor.dims(), [3]);
    
    #[cfg(feature = "wgpu-backend")]
    println!("Verified tensor creation on WGPU backend (Integration Test)");
    
    #[cfg(all(feature = "ndarray-backend", not(feature = "wgpu-backend")))]
    println!("Verified tensor creation on NdArray backend (Integration Test)");
}
