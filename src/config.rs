use crate::cli::Cli;
use crate::error::{BingrepError, Result};

/// Configuration constants and defaults for bingrep
#[derive(Debug, Clone)]
pub struct Config {
    pub buffer_size: usize,
    pub buffer_padding: usize,
    pub max_line_width: usize,
    pub min_line_width: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            buffer_size: 64 * 1024, // 64KB for better performance
            buffer_padding: 1024,   // To handle patterns across buffer boundaries
            max_line_width: 8192,   // Maximum bytes per line
            min_line_width: 1,      // Minimum bytes per line
        }
    }
}

impl Config {
    /// Validate all input parameters from CLI
    pub fn validate_cli(&self, cli: &Cli) -> Result<()> {
        // Validate line width
        if !self.validate_width(cli.line_width) {
            return Err(BingrepError::InvalidWidth(cli.line_width));
        }

        // Validate limit (must be non-negative, but usize ensures this)
        // No additional validation needed for limit

        // Validate position (must be non-negative, but u64 ensures this)
        // No additional validation needed for position

        Ok(())
    }

    pub fn validate_width(&self, width: usize) -> bool {
        width >= self.min_line_width && width <= self.max_line_width
    }

    pub fn get_buffer_size(&self, width: usize) -> usize {
        // Use smaller buffer size for small line widths to avoid memory waste
        if width < 1024 {
            std::cmp::max(width * 4, 4096) // At least 4KB, or 4x the line width
        } else {
            self.buffer_size
        }
    }

    pub fn get_min_width(&self) -> usize {
        self.min_line_width
    }

    pub fn get_max_width(&self) -> usize {
        self.max_line_width
    }
}
