use crate::allocators::allocator::Allocator;

pub struct BasicAllocator;

impl Allocator for BasicAllocator {
    fn alloc(&mut self, size: usize) -> Vec<u8> {
        vec![0_u8; size]
    }

    fn free(&mut self, buffer: Vec<u8>) {
        drop(buffer);
    }
}
