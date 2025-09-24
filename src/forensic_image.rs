//! Forensic image format support
//!
//! This module provides functionality to read various forensic image formats
//! including E01 (EWF) and VMDK files using the exhume_body library.

use crate::error::{BingrepError, Result};
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// Forensic image reader that handles E01 and VMDK forensic image files
pub struct ForensicImageReader {
    #[cfg(feature = "exhume")]
    body: exhume_body::Body,
    #[cfg(not(feature = "exhume"))]
    _placeholder: std::marker::PhantomData<()>,
    size: u64,
}

impl ForensicImageReader {
    /// Create a new forensic image reader from a file path
    ///
    /// This will automatically detect the format (E01, VMDK, etc.) and load the image
    #[cfg(feature = "exhume")]
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let path_str = path.to_str()
            .ok_or_else(|| BingrepError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid path encoding"
            )))?;

        // Try to create the exhume_body reader
        let body = match std::panic::catch_unwind(|| {
            exhume_body::Body::new(path_str.to_string(), "auto")
        }) {
            Ok(body) => body,
            Err(_) => {
                return Err(BingrepError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Failed to open forensic image: {}. The file may be corrupt, incomplete, or not a valid forensic image format.", path.display())
                )));
            }
        };

        // Get the size if possible
        let size = 0; // TODO: Get actual size from exhume_body

        Ok(ForensicImageReader {
            body,
            size,
        })
    }

    #[cfg(not(feature = "exhume"))]
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        Err(BingrepError::Io(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            format!(
                "Forensic image support not available. File: {}.\n\
                To add forensic image support:\n\
                1. Enable the 'exhume' feature in Cargo.toml\n\
                2. Install required dependencies for exhume_body",
                path.display()
            )
        )))
    }

    /// Get the total size of the forensic image
    pub fn size(&self) -> u64 {
        self.size
    }
}

#[cfg(feature = "exhume")]
impl Read for ForensicImageReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.body.read(buf)
    }
}

#[cfg(feature = "exhume")]
impl Seek for ForensicImageReader {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.body.seek(pos)
    }
}

#[cfg(not(feature = "exhume"))]
impl Read for ForensicImageReader {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Forensic image support not enabled"
        ))
    }
}

#[cfg(not(feature = "exhume"))]
impl Seek for ForensicImageReader {
    fn seek(&mut self, _pos: SeekFrom) -> std::io::Result<u64> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Forensic image support not enabled"
        ))
    }
}

/// Check if a file path has a forensic image extension (E01 or VMDK)
pub fn is_forensic_image<P: AsRef<Path>>(path: P) -> bool {
    is_e01_file(&path) || is_vmdk_file(&path)
}

/// Check if a file path has an E01 extension
pub fn is_e01_file<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase() == "e01")
        .unwrap_or(false)
}

/// Check if a file path has a VMDK extension
pub fn is_vmdk_file<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase() == "vmdk")
        .unwrap_or(false)
}

/// Get the format name for a forensic image file
pub fn get_format_name<P: AsRef<Path>>(path: P) -> Option<&'static str> {
    let path = path.as_ref();
    if is_e01_file(&path) {
        Some("E01/EWF")
    } else if is_vmdk_file(&path) {
        Some("VMDK")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_e01_file() {
        assert!(is_e01_file("test.e01"));
        assert!(is_e01_file("TEST.E01"));
        assert!(is_e01_file("/path/to/image.e01"));
        assert!(!is_e01_file("test.dd"));
        assert!(!is_e01_file("test.raw"));
        assert!(!is_e01_file("test"));
    }

    #[test]
    fn test_is_vmdk_file() {
        assert!(is_vmdk_file("test.vmdk"));
        assert!(is_vmdk_file("TEST.VMDK"));
        assert!(is_vmdk_file("/path/to/image.vmdk"));
        assert!(!is_vmdk_file("test.dd"));
        assert!(!is_vmdk_file("test.raw"));
        assert!(!is_vmdk_file("test"));
    }

    #[test]
    fn test_is_forensic_image() {
        assert!(is_forensic_image("test.e01"));
        assert!(is_forensic_image("test.vmdk"));
        assert!(is_forensic_image("TEST.E01"));
        assert!(is_forensic_image("TEST.VMDK"));
        assert!(!is_forensic_image("test.dd"));
        assert!(!is_forensic_image("test.raw"));
        assert!(!is_forensic_image("test"));
    }

    #[test]
    fn test_get_format_name() {
        assert_eq!(get_format_name("test.e01"), Some("E01/EWF"));
        assert_eq!(get_format_name("test.vmdk"), Some("VMDK"));
        assert_eq!(get_format_name("TEST.E01"), Some("E01/EWF"));
        assert_eq!(get_format_name("TEST.VMDK"), Some("VMDK"));
        assert_eq!(get_format_name("test.dd"), None);
        assert_eq!(get_format_name("test.raw"), None);
    }

    #[test]
    #[cfg(not(feature = "exhume"))]
    fn test_forensic_reader_returns_error() {
        let result = ForensicImageReader::new("test.e01");
        assert!(result.is_err());

        if let Err(BingrepError::Io(io_err)) = result {
            assert_eq!(io_err.kind(), std::io::ErrorKind::Unsupported);
        }
    }
}