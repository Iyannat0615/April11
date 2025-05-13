
use crate::memory_manager::MemoryManager;

pub fn insert(manager: &mut MemoryManager, id: u16, data: Vec<u8>) {
    match manager.insert(id, data) {
        Some(_) => println!("Data inserted with ID {}", id),
        None => println!("Failed to insert data with ID {}", id),
    }
}
