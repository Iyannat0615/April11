use crate::memory_manager::MemoryManager; // ✅ Import MemoryManager

impl MemoryManager {
    /// Finds a memory block by its ID and returns a raw pointer to the start of the block.
    ///
    /// # Parameters
    ///
    /// - `id`: The `usize` identifier of the memory block to find.
    ///
    /// # Returns
    ///
    /// - `Some(*const u8)` if the memory block with the specified ID is found.
    /// - `None` if the ID is not found.
    ///
    /// # Safety
    ///
    /// This method returns a raw pointer. The caller must handle it safely,
    /// as dereferencing raw pointers is unsafe.
    pub fn find(&self, id: usize) -> Option<*const u8> {
        // Borrow the allocated blocks and search for the block corresponding to the given ID
        let allocated_blocks = self.allocated_blocks.borrow();

        // If the block exists, return a raw pointer to its start in memory
        allocated_blocks.get(&id).map(|block| {
            // Borrow the memory and get the start of the block as a pointer
            let memory = self.memory.borrow();
            &memory[block.start] as *const u8
        })
    }
}
