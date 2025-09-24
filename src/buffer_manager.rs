use std::io::{Read, Result};

/// Buffer manager for efficient memory reuse during file processing
///
/// Manages multiple reusable buffers to minimize allocations during
/// file reading and pattern matching operations.
pub struct BufferManager {
    main_buffer: Vec<u8>,
    extra_buffer: Vec<u8>,
    temp_buffer: Vec<u8>,
}

impl BufferManager {
    /// Create a new BufferManager with specified buffer sizes
    ///
    /// # Arguments
    ///
    /// * `buffer_size` - Size of the main buffer for file reading
    /// * `max_extra_size` - Initial size of the extra buffer for overflow handling
    pub fn new(buffer_size: usize, max_extra_size: usize) -> Self {
        Self {
            main_buffer: vec![0u8; buffer_size],
            extra_buffer: vec![0u8; max_extra_size],
            temp_buffer: Vec::new(),
        }
    }

    /// Get a mutable reference to the main buffer
    ///
    /// Returns the main buffer used for primary file reading operations
    pub fn get_main_buffer(&mut self) -> &mut Vec<u8> {
        &mut self.main_buffer
    }

    /// Get a mutable reference to the extra buffer, resizing if needed
    ///
    /// Automatically resizes the extra buffer if the requested size is larger
    /// than the current capacity.
    pub fn get_extra_buffer(&mut self, needed_size: usize) -> &mut Vec<u8> {
        if self.extra_buffer.len() < needed_size {
            self.extra_buffer.resize(needed_size, 0);
        }
        &mut self.extra_buffer
    }

    /// Read data into main buffer
    ///
    /// Reads data from the given reader into the main buffer and returns
    /// the number of bytes read.
    pub fn read_into_main<R: Read>(&mut self, reader: &mut R) -> Result<usize> {
        reader.read(&mut self.main_buffer)
    }

    /// Read data into extra buffer
    ///
    /// Reads up to `size` bytes from the reader into the extra buffer,
    /// resizing it if necessary.
    pub fn read_into_extra<R: Read>(&mut self, reader: &mut R, size: usize) -> Result<usize> {
        let buffer = self.get_extra_buffer(size);
        let bytes_read = reader.read(&mut buffer[..size])?;
        Ok(bytes_read)
    }

    /// Combine data from main buffer and extra buffer into temp buffer
    ///
    /// Creates a contiguous view of data spanning both buffers, useful for
    /// handling patterns that cross buffer boundaries.
    pub fn combine_buffers(
        &mut self,
        main_start: usize,
        main_end: usize,
        extra_size: usize,
    ) -> &[u8] {
        self.temp_buffer.clear();
        self.temp_buffer
            .extend_from_slice(&self.main_buffer[main_start..main_end]);
        self.temp_buffer
            .extend_from_slice(&self.extra_buffer[..extra_size]);
        &self.temp_buffer
    }

    /// Get an immutable slice from the main buffer
    ///
    /// Returns a slice of the main buffer from `start` to `end` indices
    pub fn get_main_slice(&self, start: usize, end: usize) -> &[u8] {
        &self.main_buffer[start..end]
    }

    /// Get an immutable slice from the extra buffer
    ///
    /// Returns a slice of the extra buffer up to the specified size
    pub fn get_extra_slice(&self, size: usize) -> &[u8] {
        &self.extra_buffer[..size]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_buffer_manager_creation() {
        let manager = BufferManager::new(1024, 512);
        assert_eq!(manager.main_buffer.len(), 1024);
        assert_eq!(manager.extra_buffer.len(), 512);
    }

    #[test]
    fn test_read_into_main() {
        let mut manager = BufferManager::new(10, 5);
        let data = b"Hello World";
        let mut cursor = Cursor::new(data);

        let bytes_read = manager.read_into_main(&mut cursor).unwrap();
        assert_eq!(bytes_read, 10); // Only 10 bytes fit in buffer
        assert_eq!(&manager.main_buffer[..bytes_read], b"Hello Worl");
    }

    #[test]
    fn test_extra_buffer_resize() {
        let mut manager = BufferManager::new(10, 5);

        // Request larger buffer than initial size
        let buffer = manager.get_extra_buffer(20);
        assert_eq!(buffer.len(), 20);
    }

    #[test]
    fn test_combine_buffers() {
        let mut manager = BufferManager::new(10, 10);
        manager.main_buffer[0..5].copy_from_slice(b"Hello");
        manager.extra_buffer[0..5].copy_from_slice(b"World");

        let combined = manager.combine_buffers(0, 5, 5);
        assert_eq!(combined, b"HelloWorld");
    }
}
