use serde::{Deserialize, Serialize};
use std::io::Write;

/// Supported output formats
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    /// Default hexadecimal format
    Hex,
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// Plain text format (similar to hex but without formatting)
    Plain,
}

impl OutputFormat {
    /// Parse output format from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "hex" => Some(Self::Hex),
            "json" => Some(Self::Json),
            "csv" => Some(Self::Csv),
            "plain" => Some(Self::Plain),
            _ => None,
        }
    }
}

/// Represents a match found in the binary data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BinaryMatch {
    /// File path where the match was found
    pub file_path: String,
    /// Byte offset in the file where the match starts
    pub offset: u64,
    /// Hexadecimal representation of the matched bytes
    pub hex_data: String,
    /// Length of the match in bytes
    pub length: usize,
    /// ASCII representation of the data (if printable)
    pub ascii_data: Option<String>,
}

/// Represents a line of hex dump output
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HexDumpLine {
    /// File path of the source file
    pub file_path: String,
    /// Byte offset of this line in the file
    pub offset: u64,
    /// Hexadecimal representation of the data
    pub hex_data: String,
    /// ASCII representation of the data (if printable)
    pub ascii_data: Option<String>,
    /// Number of bytes in this line
    pub byte_count: usize,
}

/// Structured output formatter
pub struct StructuredFormatter {
    format: OutputFormat,
}

impl StructuredFormatter {
    /// Create a new structured formatter
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }

    /// Output matches in the specified format
    pub fn output_matches<W: Write>(
        &self,
        matches: &[BinaryMatch],
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.format {
            OutputFormat::Hex => self.output_matches_hex(matches, writer),
            OutputFormat::Json => self.output_matches_json(matches, writer),
            OutputFormat::Csv => self.output_matches_csv(matches, writer),
            OutputFormat::Plain => self.output_matches_plain(matches, writer),
        }
    }

    /// Output hex dump lines in the specified format
    pub fn output_hex_dump<W: Write>(
        &self,
        lines: &[HexDumpLine],
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.format {
            OutputFormat::Hex => self.output_hex_dump_hex(lines, writer),
            OutputFormat::Json => self.output_hex_dump_json(lines, writer),
            OutputFormat::Csv => self.output_hex_dump_csv(lines, writer),
            OutputFormat::Plain => self.output_hex_dump_plain(lines, writer),
        }
    }

    /// Output matches in hex format (default)
    fn output_matches_hex<W: Write>(
        &self,
        matches: &[BinaryMatch],
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for m in matches {
            let hex_offset_length = format!("{:X}", m.offset).len();
            writeln!(
                writer,
                "{:0width$X}h : {}",
                m.offset,
                m.hex_data,
                width = hex_offset_length
            )?;
        }
        Ok(())
    }

    /// Output matches in JSON format
    fn output_matches_json<W: Write>(
        &self,
        matches: &[BinaryMatch],
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        serde_json::to_writer_pretty(&mut *writer, matches)?;
        writeln!(writer)?;
        Ok(())
    }

    /// Output matches in CSV format
    fn output_matches_csv<W: Write>(
        &self,
        matches: &[BinaryMatch],
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut csv_writer = csv::Writer::from_writer(writer);

        // Write header
        csv_writer.write_record(&["file_path", "offset", "hex_data", "length", "ascii_data"])?;

        // Write data
        for m in matches {
            csv_writer.write_record(&[
                &m.file_path,
                &m.offset.to_string(),
                &m.hex_data,
                &m.length.to_string(),
                &m.ascii_data.as_ref().unwrap_or(&"".to_string()),
            ])?;
        }

        csv_writer.flush()?;
        Ok(())
    }

    /// Output matches in plain format
    fn output_matches_plain<W: Write>(
        &self,
        matches: &[BinaryMatch],
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for m in matches {
            writeln!(writer, "{}:{} {}", m.file_path, m.offset, m.hex_data)?;
        }
        Ok(())
    }

    /// Output hex dump in hex format (default)
    fn output_hex_dump_hex<W: Write>(
        &self,
        lines: &[HexDumpLine],
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for line in lines {
            let hex_offset_length = format!("{:X}", line.offset).len();
            writeln!(
                writer,
                "{:0width$X}h : {}",
                line.offset,
                line.hex_data,
                width = hex_offset_length
            )?;
        }
        Ok(())
    }

    /// Output hex dump in JSON format
    fn output_hex_dump_json<W: Write>(
        &self,
        lines: &[HexDumpLine],
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        serde_json::to_writer_pretty(&mut *writer, lines)?;
        writeln!(writer)?;
        Ok(())
    }

    /// Output hex dump in CSV format
    fn output_hex_dump_csv<W: Write>(
        &self,
        lines: &[HexDumpLine],
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut csv_writer = csv::Writer::from_writer(writer);

        // Write header
        csv_writer.write_record(&[
            "file_path",
            "offset",
            "hex_data",
            "byte_count",
            "ascii_data",
        ])?;

        // Write data
        for line in lines {
            csv_writer.write_record(&[
                &line.file_path,
                &line.offset.to_string(),
                &line.hex_data,
                &line.byte_count.to_string(),
                &line.ascii_data.as_ref().unwrap_or(&"".to_string()),
            ])?;
        }

        csv_writer.flush()?;
        Ok(())
    }

    /// Output hex dump in plain format
    fn output_hex_dump_plain<W: Write>(
        &self,
        lines: &[HexDumpLine],
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for line in lines {
            writeln!(
                writer,
                "{}:{} {}",
                line.file_path, line.offset, line.hex_data
            )?;
        }
        Ok(())
    }
}

/// Helper functions for creating structured data
impl BinaryMatch {
    /// Create a new BinaryMatch
    pub fn new(file_path: String, offset: u64, hex_data: String, length: usize) -> Self {
        let ascii_data = Self::bytes_to_ascii_if_printable(&hex_data);
        Self {
            file_path,
            offset,
            hex_data,
            length,
            ascii_data,
        }
    }

    /// Convert hex string to ASCII if all bytes are printable
    fn bytes_to_ascii_if_printable(hex_data: &str) -> Option<String> {
        let hex_bytes: Result<Vec<u8>, _> = hex_data
            .split_whitespace()
            .map(|hex_byte| u8::from_str_radix(hex_byte, 16))
            .collect();

        match hex_bytes {
            Ok(bytes) => {
                if bytes
                    .iter()
                    .all(|&b| b.is_ascii() && (b.is_ascii_graphic() || b == b' '))
                {
                    Some(String::from_utf8_lossy(&bytes).to_string())
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl HexDumpLine {
    /// Create a new HexDumpLine
    pub fn new(file_path: String, offset: u64, hex_data: String, byte_count: usize) -> Self {
        let ascii_data = BinaryMatch::bytes_to_ascii_if_printable(&hex_data);
        Self {
            file_path,
            offset,
            hex_data,
            byte_count,
            ascii_data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_from_str() {
        assert!(matches!(
            OutputFormat::from_str("hex"),
            Some(OutputFormat::Hex)
        ));
        assert!(matches!(
            OutputFormat::from_str("json"),
            Some(OutputFormat::Json)
        ));
        assert!(matches!(
            OutputFormat::from_str("csv"),
            Some(OutputFormat::Csv)
        ));
        assert!(matches!(
            OutputFormat::from_str("plain"),
            Some(OutputFormat::Plain)
        ));
        assert!(matches!(OutputFormat::from_str("invalid"), None));
    }

    #[test]
    fn test_binary_match_creation() {
        let m = BinaryMatch::new(
            "test.bin".to_string(),
            0x100,
            "48 65 6C 6C 6F".to_string(), // "Hello"
            5,
        );

        assert_eq!(m.file_path, "test.bin");
        assert_eq!(m.offset, 0x100);
        assert_eq!(m.length, 5);
        assert_eq!(m.ascii_data, Some("Hello".to_string()));
    }

    #[test]
    fn test_json_output() {
        let matches = vec![BinaryMatch::new(
            "test.bin".to_string(),
            0,
            "48 65 6C 6C 6F".to_string(),
            5,
        )];

        let formatter = StructuredFormatter::new(OutputFormat::Json);
        let mut output = Vec::new();
        formatter.output_matches(&matches, &mut output).unwrap();

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("test.bin"));
        assert!(output_str.contains("48 65 6C 6C 6F"));
    }
}
