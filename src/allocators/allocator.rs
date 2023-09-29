pub trait Allocator: Send {
    fn alloc(&mut self, size: usize) -> Vec<u8>;

    fn free(&mut self, buffer: Vec<u8>);
}
