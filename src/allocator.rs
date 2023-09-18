pub trait Allocator {
    fn alloc(&mut self, size: usize) -> Vec<u8>;

    fn free(&mut self, buffer: Vec<u8>);
}

pub struct SimpleAllocator;

impl Allocator for SimpleAllocator {
    fn alloc(&mut self, size: usize) -> Vec<u8> {
        vec![0_u8; size]
    }

    fn free(&mut self, buffer: Vec<u8>) {
        drop(buffer);
    }
}
