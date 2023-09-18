use crate::allocator::Allocator;
use std::collections::VecDeque;

pub struct BufferAllocator {
    buffers: VecDeque<Vec<u8>>,
    buffer_size: usize,
}

impl BufferAllocator {
    pub fn new(buffer_size: usize) -> Self {
        BufferAllocator {
            buffers: VecDeque::new(),
            buffer_size,
        }
    }
}

impl Allocator for BufferAllocator {
    fn alloc(&mut self, _: usize) -> Vec<u8> {
        if let Some(buffer) = self.buffers.pop_front() {
            buffer
        } else {
            println!("ALLOC");
            vec![0_u8; self.buffer_size]
        }
    }

    fn free(&mut self, buffer: Vec<u8>) {
        self.buffers.push_back(buffer);
    }
}
