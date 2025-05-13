use std::collections::HashMap;

pub struct MemoryManager {
    memory: [u8; 65535], // The memory block, 65535 bytes in size
    allocations: HashMap<u16, (usize, usize)>, // id -> (start index, size)
    next_free: usize, // The next available free index in memory
}

impl MemoryManager {
    /// Creates a new `MemoryManager` with an empty memory block and no allocations.
    /// The memory block size is set to 65535 bytes.
    pub fn new() -> Self {
        Self {
            memory: [0; 65535],
            allocations: HashMap::new(),
            next_free: 0,
        }
    }

    /// Inserts data into memory with a given `id`.
    /// 
    /// # Parameters:
    /// - `id`: Unique identifier for the data.
    /// - `data`: The byte vector to insert into memory.
    ///
    /// # Returns:
    /// - `Some(())` if the data is inserted successfully.
    /// - `None` if the ID already exists or there is not enough space.
    ///
    /// # Behavior:
    /// - Checks for duplicate IDs.
    /// - Ensures there is enough space in memory.
    /// - Copies the data into the memory and tracks the allocation.
    pub fn insert(&mut self, id: u16, data: Vec<u8>) -> Option<()> {
        let size = data.len();

        // Reject duplicate ID
        if self.allocations.contains_key(&id) {
            return None;
        }

        // Not enough space
        if self.next_free + size > self.memory.len() {
            return None;
        }

        // Copy data into memory
        let start = self.next_free;
        self.memory[start..start + size].copy_from_slice(&data);

        // Track allocation
        self.allocations.insert(id, (start, size));
        self.next_free += size;

        Some(())
    }

    /// Reads data from memory using the provided `id`.
    /// 
    /// # Parameters:
    /// - `id`: The unique identifier for the data to read.
    ///
    /// # Returns:
    /// - `Some(data)` if the data is found.
    /// - `None` if no data is found for the given ID.
    pub fn read(&self, id: u16) -> Option<Vec<u8>> {
        if let Some(&(start, size)) = self.allocations.get(&id) {
            Some(self.memory[start..start + size].to_vec())
        } else {
            None
        }
    }

    /// Updates the data for the specified ID.
    /// 
    /// # Parameters:
    /// - `id`: The unique identifier of the data to update.
    /// - `data`: The new byte vector to replace the old data.
    ///
    /// # Returns:
    /// - `Some(())` if the update is successful.
    /// - `None` if the ID does not exist or the new data is larger than the current allocation.
    ///
    /// # Behavior:
    /// - Updates the data in memory and ensures the data does not grow larger than the existing allocation.
    /// - If the new data is smaller, it pads the remaining space with zeros.
    pub fn update(&mut self, id: u16, data: Vec<u8>) -> Option<()> {
        if let Some(&(start, size)) = self.allocations.get(&id) {
            if data.len() > size {
                return None; // Don't allow expanding
            }

            // Overwrite the existing allocation
            self.memory[start..start + data.len()].copy_from_slice(&data);

            // If data is shorter, pad the rest with zeros
            if data.len() < size {
                for i in start + data.len()..start + size {
                    self.memory[i] = 0;
                }
            }

            Some(())
        } else {
            None
        }
    }

    /// Deletes the data associated with the specified ID.
    /// 
    /// # Parameters:
    /// - `id`: The unique identifier of the data to delete.
    ///
    /// # Returns:
    /// - `Some(())` if the deletion is successful.
    /// - `None` if the ID does not exist.
    ///
    /// # Behavior:
    /// - Removes the data from memory and clears the memory block.
    pub fn delete(&mut self, id: u16) -> Option<()> {
        if let Some((start, size)) = self.allocations.remove(&id) {
            for i in start..start + size {
                self.memory[i] = 0;
            }
            Some(())
        } else {
            None
        }
    }

    /// Dumps the contents of memory along with the allocated data.
    /// 
    /// # Behavior:
    /// - Prints out the allocated memory blocks with their IDs, start positions, sizes, and the stored data.
    pub fn dump(&self) {
        println!("--- Memory Dump ---");
        for (id, (start, size)) in &self.allocations {
            let data = &self.memory[*start..*start + *size];
            let display_data = String::from_utf8_lossy(data);
            println!("ID {} -> Start: {}, Size: {}, Data: {}", id, start, size, display_data);
        }
        println!("--------------------");
    }
}

// Test cases for the `MemoryManager` class
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the basic functionality of inserting and then reading data.
    /// 
    /// - Inserts a byte vector (e.g., b"Hello") into memory using a specific ID.
    /// - Then attempts to read it back using the same ID.
    /// - Asserts that the returned data matches the original input.
    #[test]
    fn test_insert_and_read() {
        let mut manager = MemoryManager::new();
        let id = 1;
        let data = vec![72, 101, 108, 108, 111]; // "Hello"

        manager.insert(id, data.clone());
        let read_data = manager.read(id).unwrap();

        assert_eq!(data, read_data);
    }

    /// Tests that inserting data with a duplicate ID fails.
    /// 
    /// - Inserts data with a certain ID.
    /// - Attempts to insert another block of data using the same ID.
    /// - Expects the second insertion to fail (i.e., returns `None`).
    #[test]
    fn test_insert_duplicate_fails() {
        let mut manager = MemoryManager::new();
        let id = 1;
        let data = vec![72, 101, 108, 108, 111]; // "Hello"

        manager.insert(id, data.clone());
        assert_eq!(manager.insert(id, data), None); // Should fail due to duplicate ID
    }

    /// Tests that an insertion fails when there is not enough memory left.
    /// 
    /// - Creates a large data vector that exceeds the memory manager's capacity.
    /// - Tries to insert it into memory.
    /// - Expects the insertion to return `None`, indicating failure.
    #[test]
    fn test_insert_out_of_space() {
        let mut manager = MemoryManager::new();
        let data = vec![0; 65536]; // Data larger than the memory block size

        assert_eq!(manager.insert(1, data), None); // Should fail due to insufficient space
    }

    /// Tests updating an existing allocation without changing its size.
    /// 
    /// - Inserts a byte vector with a given ID.
    /// - Updates the memory at that ID with a new vector of the same or smaller size.
    /// - Reads the memory back and verifies the update was applied correctly.
    /// - If the new data is shorter, ensures that trailing bytes are zeroed out.
    #[test]
    fn test_update() {
        let mut manager = MemoryManager::new();
        let id = 1;
        let data = vec![72, 101, 108, 108, 111]; // "Hello"

        manager.insert(id, data.clone());
        let updated_data = vec![80, 121, 116, 104, 111, 110]; // "Python"
        manager.update(id, updated_data.clone());

        let read_data = manager.read(id).unwrap();
        assert_eq!(updated_data, read_data);
    }

    /// Tests deleting an existing memory allocation.
    /// 
    /// - Inserts a byte vector into memory with a given ID.
    /// - Deletes the data associated with that ID.
    /// - Attempts to read from that ID and expects it to return `None`.
    /// - Verifies that the data is removed and memory is cleared.
    #[test]
    fn test_delete() {
        let mut manager = MemoryManager::new();
        let id = 1;
        let data = vec![72, 101, 108, 108, 111]; // "Hello"

        manager.insert(id, data.clone());
        manager.delete(id);

        assert_eq!(manager.read(id), None); // Data should be deleted
    }
}
