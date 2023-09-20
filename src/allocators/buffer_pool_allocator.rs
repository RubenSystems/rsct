use crate::allocators::allocator::Allocator;
use std::collections::VecDeque;

pub struct BufferPoolAllocator {
    buffers: VecDeque<Vec<u8>>,
    buffer_size: usize,
}

impl BufferPoolAllocator {
    pub fn new(buffer_size: usize) -> Self {
        BufferPoolAllocator {
            buffers: VecDeque::new(),
            buffer_size,
        }
    }
}

impl Allocator for BufferPoolAllocator {
    fn alloc(&mut self, _: usize) -> Vec<u8> {
        if let Some(buffer) = self.buffers.pop_front() {
            buffer
        } else {
            vec![0_u8; self.buffer_size]
        }
    }

    fn free(&mut self, buffer: Vec<u8>) {
        self.buffers.push_back(buffer);
    }
}
