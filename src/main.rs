use hxgrep::cli::Cli;
use hxgrep::config::Config;
use hxgrep::error::Result;
use hxgrep::multifile::MultiFileProcessor;
use hxgrep::output::OutputFormatter;
use hxgrep::parallel::{ParallelHexDump, ParallelProcessor};
use hxgrep::regex_processor::RegexProcessor;
use hxgrep::stream::FileProcessor;
use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set global color choice
    hxgrep::color_context::set_color_choice(cli.color.clone());

    // Check file path or stdin
    let file_path = match &cli.file_path {
        Some(path) => {
            if path == "-" {
                // Handle stdin input
                return handle_stdin_input(&cli);
            }
            path.clone()
        }
        None => {
            // Clap will automatically show help when no file path is provided
            eprintln!("Error: 파일 경로가 필요합니다.\n");
            eprintln!("사용법: hxgrep <파일경로> [옵션]");
            eprintln!("사용법: hxgrep - [옵션] < input_file (stdin)");
            eprintln!("도움말: hxgrep --help");
            return Ok(());
        }
    };

    // Handle multi-file processing
    if cli.multi_file {
        let config = Config::default();
        config.validate_cli(&cli)?;

        let multi_processor = MultiFileProcessor::new(config);

        return multi_processor.process_files_by_glob(
            &file_path,
            cli.expression.as_deref(),
            cli.line_width,
            cli.limit,
            &cli.separator,
            !cli.hide_offset,
            cli.parallel,
            cli.chunk_size,
            cli.global_limit,
        );
    }

    // Create configuration and validate CLI parameters
    let config = Config::default();
    config.validate_cli(&cli)?;

    let mut processor = FileProcessor::new(config);

    // Check if this is a forensic image file (E01, VMDK) and handle accordingly
    if hxgrep::forensic_image::is_forensic_image(&file_path) {
        // Process forensic image file - parallel processing not supported for forensic images yet
        let format_name = hxgrep::forensic_image::get_format_name(&file_path)
            .unwrap_or("Unknown");
        eprintln!("Detected {} forensic image: {}", format_name, file_path);

        if let Some(expression) = cli.expression {
            let regex = RegexProcessor::compile_pattern(&expression)?;
            processor.process_stream_by_regex_from_path(
                &file_path,
                &regex,
                cli.line_width,
                cli.limit,
                &cli.separator,
                !cli.hide_offset,
            )?;
        } else {
            processor.process_file_stream_from_path(
                &file_path,
                cli.line_width,
                cli.limit,
                &cli.separator,
                !cli.hide_offset,
            )?;
        }
    } else {
        // Open regular file
        let mut file = File::open(&file_path)?;
        let file_size = file.metadata()?.len();

        // Seek to starting position
        file.seek(SeekFrom::Start(cli.position))?;

        // Process file with or without regex
        if let Some(expression) = cli.expression {
            let regex = RegexProcessor::compile_pattern(&expression)?;

            if cli.parallel && file_size > cli.chunk_size as u64 {
                // Use parallel processing for large files
                ParallelProcessor::process_file_parallel(
                    &mut file,
                    &regex,
                    cli.chunk_size,
                    cli.line_width,
                    cli.limit,
                    &cli.separator,
                    !cli.hide_offset,
                    file_size,
                )?;
            } else {
                // Use regular processing
                processor.process_stream_by_regex(
                    &mut file,
                    &regex,
                    cli.line_width,
                    cli.limit,
                    &cli.separator,
                    !cli.hide_offset,
                )?;
            }
        } else {
            if cli.parallel && file_size > cli.chunk_size as u64 {
                // Use parallel processing for hex dump
                ParallelHexDump::process_file_parallel(
                    &mut file,
                    cli.chunk_size,
                    cli.line_width,
                    cli.limit,
                    &cli.separator,
                    !cli.hide_offset,
                    file_size,
                )?;
            } else {
                // Use regular processing
                processor.process_file_stream(
                    &mut file,
                    cli.line_width,
                    cli.limit,
                    &cli.separator,
                    !cli.hide_offset,
                    file_size,
                )?;
            }
        }
    }

    Ok(())
}

/// Handle stdin input processing
fn handle_stdin_input(cli: &Cli) -> Result<()> {
    let config = Config::default();
    config.validate_cli(cli)?;

    // Read all data from stdin into a buffer
    let mut stdin_data = Vec::new();
    io::stdin().read_to_end(&mut stdin_data)?;

    if stdin_data.is_empty() {
        eprintln!("Warning: No data received from stdin");
        return Ok(());
    }

    let data_size = stdin_data.len() as u64;

    // Process data with or without regex
    if let Some(expression) = &cli.expression {
        let regex = RegexProcessor::compile_pattern(expression)?;
        process_stdin_with_regex(&stdin_data, &regex, cli, data_size)?;
    } else {
        process_stdin_hex_dump(&stdin_data, cli, data_size)?;
    }

    Ok(())
}

/// Process stdin data with regex search
fn process_stdin_with_regex(
    data: &[u8],
    regex: &regex::bytes::Regex,
    cli: &Cli,
    data_size: u64,
) -> Result<()> {
    let hex_offset_length = OutputFormatter::calculate_hex_offset_length(data_size);
    let mut match_count = 0;

    for mat in regex.find_iter(data) {
        let match_offset = mat.start() as u64;
        let end_pos = (mat.start() + cli.line_width).min(data.len());
        let display_bytes = &data[mat.start()..end_pos];

        let hex_string = OutputFormatter::format_bytes_as_hex(display_bytes, &cli.separator);
        OutputFormatter::print_line(
            match_offset,
            &hex_string,
            !cli.hide_offset,
            hex_offset_length,
        );

        match_count += 1;
        if cli.limit > 0 && match_count >= cli.limit {
            break;
        }
    }

    Ok(())
}

/// Process stdin data as hex dump
fn process_stdin_hex_dump(data: &[u8], cli: &Cli, data_size: u64) -> Result<()> {
    let hex_offset_length = OutputFormatter::calculate_hex_offset_length(data_size);
    let mut pos = 0;
    let mut line = 0;

    while pos < data.len() {
        let end_pos = (pos + cli.line_width).min(data.len());
        let line_bytes = &data[pos..end_pos];

        let hex_string = OutputFormatter::format_bytes_as_hex(line_bytes, &cli.separator);
        OutputFormatter::print_line(pos as u64, &hex_string, !cli.hide_offset, hex_offset_length);

        pos += cli.line_width;
        line += 1;

        if cli.limit > 0 && line >= cli.limit {
            break;
        }
    }

    Ok(())
}
