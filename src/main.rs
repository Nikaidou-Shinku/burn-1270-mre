use burn::{
  backend::{wgpu::WgpuDevice, Wgpu},
  module::Module,
  nn::conv::Conv2d,
  record::{FullPrecisionSettings, Recorder},
  tensor::backend::Backend,
};
use burn_import::pytorch::PyTorchFileRecorder;

#[derive(Debug, Module)]
struct Foo<B: Backend> {
  conv0: Conv2d<B>,
}

fn main() {
  let device = WgpuDevice::default();
  let record: FooRecord<Wgpu> = PyTorchFileRecorder::<FullPrecisionSettings>::default()
    .load("foo.pt".into(), &device)
    .unwrap();
  dbg!(record.conv0.weight);
}
