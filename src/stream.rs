use crate::buffer_manager::BufferManager;
use crate::config::Config;
use crate::error::Result;
use crate::forensic_image::{ForensicImageReader, is_forensic_image};
use crate::output::OutputFormatter;
use regex::bytes::Regex;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// File processor for handling binary file searching and hex dump operations
pub struct FileProcessor {
    config: Config,
    buffer_manager: BufferManager,
}

impl FileProcessor {
    /// Create a new FileProcessor with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration settings for buffer sizes and limits
    pub fn new(config: Config) -> Self {
        let buffer_size = config.buffer_size;
        let max_extra_size = config.max_line_width.max(1024); // At least 1KB for extra buffer
        let buffer_manager = BufferManager::new(buffer_size, max_extra_size);

        Self {
            config,
            buffer_manager,
        }
    }

    /// Process file without regex - simple hex dump
    ///
    /// Reads a file and outputs its contents in hexadecimal format.
    /// Automatically detects forensic image files (E01, VMDK) and processes them using appropriate libraries.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file to read from
    /// * `width` - Number of bytes to display per line
    /// * `limit` - Maximum number of lines to output (0 for unlimited)
    /// * `separator` - String to separate hex bytes
    /// * `show_offset` - Whether to display offset values
    pub fn process_file_stream_from_path<P: AsRef<Path>>(
        &mut self,
        file_path: P,
        width: usize,
        limit: usize,
        separator: &str,
        show_offset: bool,
    ) -> Result<()> {
        let file_path = file_path.as_ref();

        if is_forensic_image(&file_path) {
            // Process forensic image file (E01, VMDK)
            let mut forensic_reader = ForensicImageReader::new(&file_path)?;
            let file_size = forensic_reader.size();
            self.process_reader_stream(&mut forensic_reader, width, limit, separator, show_offset, file_size)
        } else {
            // Process regular file
            let mut file = File::open(&file_path)?;
            let file_size = file.metadata()?.len();
            self.process_reader_stream(&mut file, width, limit, separator, show_offset, file_size)
        }
    }

    /// Process file without regex - simple hex dump
    ///
    /// Reads a file and outputs its contents in hexadecimal format.
    ///
    /// # Arguments
    ///
    /// * `file` - File to read from
    /// * `width` - Number of bytes to display per line
    /// * `limit` - Maximum number of lines to output (0 for unlimited)
    /// * `separator` - String to separate hex bytes
    /// * `show_offset` - Whether to display offset values
    /// * `file_size` - Total size of the file for offset formatting
    pub fn process_file_stream(
        &mut self,
        file: &mut File,
        width: usize,
        limit: usize,
        separator: &str,
        show_offset: bool,
        file_size: u64,
    ) -> Result<()> {
        self.process_reader_stream(file, width, limit, separator, show_offset, file_size)
    }

    /// Generic stream processing function that works with any Read + Seek reader
    fn process_reader_stream<R: Read + Seek>(
        &mut self,
        reader: &mut R,
        width: usize,
        limit: usize,
        separator: &str,
        show_offset: bool,
        file_size: u64,
    ) -> Result<()> {
        let mut pos = reader.stream_position()?;
        let mut line = 0;
        let hex_offset_length = OutputFormatter::calculate_hex_offset_length(file_size);

        // Get a reusable buffer of the right size
        let buffer = self.buffer_manager.get_extra_buffer(width);

        loop {
            let bytes_read = reader.read(&mut buffer[..width])?;
            if bytes_read == 0 {
                break;
            }

            line += 1;

            let hex_string = OutputFormatter::format_bytes_as_hex(&buffer[..bytes_read], separator);
            OutputFormatter::print_line(pos, &hex_string, show_offset, hex_offset_length);

            pos += bytes_read as u64;

            // Check line limit
            if limit > 0 && line >= limit {
                break;
            }
        }

        Ok(())
    }

    /// Process file with regex pattern matching from file path
    ///
    /// Searches a file for regex pattern matches and outputs matching regions.
    /// Automatically detects forensic image files (E01, VMDK) and processes them using appropriate libraries.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file to search in
    /// * `regex` - Compiled regex pattern to search for
    /// * `width` - Number of bytes to display per match
    /// * `limit` - Maximum number of matches to output (0 for unlimited)
    /// * `separator` - String to separate hex bytes
    /// * `show_offset` - Whether to display offset values
    pub fn process_stream_by_regex_from_path<P: AsRef<Path>>(
        &mut self,
        file_path: P,
        regex: &Regex,
        width: usize,
        limit: usize,
        separator: &str,
        show_offset: bool,
    ) -> Result<()> {
        let file_path = file_path.as_ref();

        if is_forensic_image(&file_path) {
            // Process forensic image file (E01, VMDK)
            let mut forensic_reader = ForensicImageReader::new(&file_path)?;
            self.process_reader_by_regex(&mut forensic_reader, regex, width, limit, separator, show_offset)
        } else {
            // Process regular file
            let mut file = File::open(&file_path)?;
            self.process_reader_by_regex(&mut file, regex, width, limit, separator, show_offset)
        }
    }

    /// Process file with regex pattern matching
    ///
    /// Searches a file for regex pattern matches and outputs matching regions.
    ///
    /// # Arguments
    ///
    /// * `file` - File to search in
    /// * `regex` - Compiled regex pattern to search for
    /// * `width` - Number of bytes to display per match
    /// * `limit` - Maximum number of matches to output (0 for unlimited)
    /// * `separator` - String to separate hex bytes
    /// * `show_offset` - Whether to display offset values
    pub fn process_stream_by_regex(
        &mut self,
        file: &mut File,
        regex: &Regex,
        width: usize,
        limit: usize,
        separator: &str,
        show_offset: bool,
    ) -> Result<()> {
        self.process_reader_by_regex(file, regex, width, limit, separator, show_offset)
    }

    /// Generic regex processing function that works with any Read + Seek reader
    fn process_reader_by_regex<R: Read + Seek>(
        &mut self,
        reader: &mut R,
        regex: &Regex,
        width: usize,
        limit: usize,
        separator: &str,
        show_offset: bool,
    ) -> Result<()> {
        let buffer_size = self.config.get_buffer_size(width);
        let buffer_padding = self.config.buffer_padding;

        let mut line = 0;
        let mut last_hit_pos: i64 = -1;

        // For EWF files, we need to get size differently
        // For now, we'll use a large default for generic readers
        let file_size = 1024 * 1024 * 1024 * 1024u64; // 1TB default
        let hex_offset_length = OutputFormatter::calculate_hex_offset_length(file_size);

        loop {
            let start_offset = reader.stream_position()?;
            let bytes_read = self.buffer_manager.read_into_main(reader)?;

            if bytes_read == 0 {
                break;
            }

            // Process regex matches directly without collecting into vector
            let buffer_slice = self.buffer_manager.get_main_slice(0, bytes_read);
            let mut matches_to_process = Vec::new();

            // Only collect match positions that we actually need to process
            for mat in regex.find_iter(buffer_slice) {
                let match_start = mat.start();
                let new_hit_pos = start_offset + match_start as u64;

                // Skip duplicates early
                if new_hit_pos as i64 > last_hit_pos {
                    matches_to_process.push(match_start);
                    // Limit collection for memory efficiency
                    if limit > 0 && matches_to_process.len() >= limit - line {
                        break;
                    }
                }
            }

            for match_start in matches_to_process {
                let new_hit_pos = start_offset + match_start as u64;

                // Prevent duplicates
                if new_hit_pos as i64 <= last_hit_pos {
                    continue;
                }

                // Handle buffer overflow - seek to match position if needed
                if match_start + width > bytes_read && bytes_read == buffer_size {
                    reader.seek(SeekFrom::Start(new_hit_pos))?;
                    last_hit_pos = new_hit_pos as i64;
                    break;
                }

                line += 1;

                // Read width bytes from match position
                let (hex_string, match_info) = self.read_match_data_with_highlight(
                    reader,
                    match_start,
                    width,
                    bytes_read,
                    start_offset,
                    separator,
                    &regex,
                )?;

                // Calculate match position within the displayed hex string
                let match_byte_pos = if match_start < width { Some(0) } else { None };
                let match_byte_len = if match_byte_pos.is_some() {
                    match_info.map(|len| std::cmp::min(len, width))
                } else {
                    None
                };

                OutputFormatter::print_line_with_match_highlight(
                    new_hit_pos,
                    &hex_string,
                    show_offset,
                    hex_offset_length,
                    crate::color_context::get_color_choice(),
                    match_byte_pos,
                    match_byte_len,
                );
                last_hit_pos = new_hit_pos as i64;

                // Check line limit
                if limit > 0 && line >= limit {
                    return Ok(());
                }
            }

            // Read next buffer with overlap to handle patterns spanning boundaries
            if bytes_read == buffer_size {
                let new_pos = reader
                    .stream_position()?
                    .saturating_sub(buffer_padding as u64);
                reader.seek(SeekFrom::Start(new_pos))?;
            }
        }

        Ok(())
    }

    /// Read match data, handling cases where width extends beyond buffer
    #[allow(dead_code)]
    fn read_match_data(
        &mut self,
        file: &mut File,
        match_start: usize,
        width: usize,
        bytes_read: usize,
        start_offset: u64,
        separator: &str,
    ) -> Result<String> {
        self.read_match_data_generic(file, match_start, width, bytes_read, start_offset, separator)
    }

    /// Read match data with highlighting information
    fn read_match_data_with_highlight<R: Read + Seek>(
        &mut self,
        reader: &mut R,
        match_start: usize,
        width: usize,
        bytes_read: usize,
        start_offset: u64,
        separator: &str,
        regex: &Regex,
    ) -> Result<(String, Option<usize>)> {
        let hex_string = self.read_match_data_generic(reader, match_start, width, bytes_read, start_offset, separator)?;

        // Find the match length by re-running the regex on the data we're about to display
        let display_start = start_offset + match_start as u64;
        let current_pos = reader.stream_position()?;
        reader.seek(SeekFrom::Start(display_start))?;

        let mut display_buffer = vec![0u8; width];
        let actual_read = reader.read(&mut display_buffer)?;
        reader.seek(SeekFrom::Start(current_pos))?;

        let match_len = if let Some(mat) = regex.find(&display_buffer[..actual_read]) {
            Some(mat.len())
        } else {
            None
        };

        Ok((hex_string, match_len))
    }

    /// Generic read match data function that works with any Read + Seek reader
    fn read_match_data_generic<R: Read + Seek>(
        &mut self,
        reader: &mut R,
        match_start: usize,
        width: usize,
        bytes_read: usize,
        start_offset: u64,
        separator: &str,
    ) -> Result<String> {
        let end_pos = std::cmp::min(match_start + width, bytes_read);
        let actual_width = end_pos - match_start;

        if actual_width < width && match_start + width > bytes_read {
            // Need to read additional data from reader
            let current_pos = reader.stream_position()?;
            reader.seek(SeekFrom::Start(start_offset + end_pos as u64))?;

            let extra_needed = width - actual_width;
            let extra_read = self.buffer_manager.read_into_extra(reader, extra_needed)?;

            // Combine data using buffer manager
            let combined_data =
                self.buffer_manager
                    .combine_buffers(match_start, end_pos, extra_read);

            reader.seek(SeekFrom::Start(current_pos))?;

            Ok(OutputFormatter::format_bytes_as_hex(
                combined_data,
                separator,
            ))
        } else {
            let main_slice = self.buffer_manager.get_main_slice(match_start, end_pos);
            Ok(OutputFormatter::format_bytes_as_hex(main_slice, separator))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_processor_creation() {
        let config = Config::default();
        let processor = FileProcessor::new(config);
        assert_eq!(processor.config.buffer_size, 64 * 1024);
    }

    #[test]
    fn test_process_file_stream() -> Result<()> {
        let config = Config::default();
        let mut processor = FileProcessor::new(config);

        // Create a temporary file with test data
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Hello World!").unwrap();
        temp_file.seek(SeekFrom::Start(0)).unwrap();

        let mut file = temp_file.reopen().unwrap();
        let file_size = file.metadata()?.len();

        // This would normally print, but in tests we just verify it doesn't error
        let result = processor.process_file_stream(&mut file, 16, 1, " ", false, file_size);
        assert!(result.is_ok());

        Ok(())
    }
}
