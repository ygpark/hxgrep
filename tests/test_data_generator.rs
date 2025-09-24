use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// 테스트 데이터 생성 도구
pub struct TestDataGenerator;

impl TestDataGenerator {
    /// CCTV 시그니처가 포함된 테스트 파일 생성
    pub fn create_cctv_test_file() -> PathBuf {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("cctv_test.bin");
        let mut file = File::create(&file_path).unwrap();

        // 일반 데이터
        file.write_all(b"Some random data before signature...")
            .unwrap();

        // H.264 NAL Unit 시그니처 예제
        file.write_all(b"\x00\x00\x00\x01\x67").unwrap(); // SPS
        file.write_all(b"sps_data_here").unwrap();

        file.write_all(b"\x00\x00\x00\x01\x68").unwrap(); // PPS
        file.write_all(b"pps_data_here").unwrap();

        file.write_all(b"\x00\x00\x00\x01\x65").unwrap(); // IDR
        file.write_all(b"idr_frame_data").unwrap();

        // 더 많은 랜덤 데이터
        for i in 0..1000 {
            file.write_all(&[i as u8]).unwrap();
        }

        // 또 다른 시그니처
        file.write_all(b"\x00\x00\x00\x01\x67").unwrap();
        file.write_all(b"another_sps").unwrap();

        file_path
    }

    /// 대용량 테스트 파일 생성 (패턴 포함)
    pub fn create_large_test_file(size_mb: usize) -> PathBuf {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(format!("large_test_{}mb.bin", size_mb));
        let mut file = File::create(&file_path).unwrap();

        let chunk_size = 1024 * 1024; // 1MB
        let pattern = b"\xDE\xAD\xBE\xEF";

        for mb in 0..size_mb {
            let mut chunk = vec![0u8; chunk_size];

            // 각 MB마다 다른 패턴 채우기
            for i in 0..chunk_size {
                chunk[i] = ((i + mb * chunk_size) % 256) as u8;
            }

            // 각 MB의 중간에 패턴 삽입
            if mb % 2 == 0 {
                chunk[chunk_size / 2..chunk_size / 2 + 4].copy_from_slice(pattern);
            }

            file.write_all(&chunk).unwrap();
        }

        file_path
    }

    /// 여러 인코딩이 섞인 테스트 파일 생성
    pub fn create_mixed_encoding_file() -> PathBuf {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("mixed_encoding.bin");
        let mut file = File::create(&file_path).unwrap();

        // ASCII 텍스트
        file.write_all(b"ASCII Text: Hello World!\n").unwrap();

        // UTF-8 한글
        file.write_all("한글 텍스트: 안녕하세요\n".as_bytes())
            .unwrap();

        // 바이너리 데이터
        file.write_all(&[0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD])
            .unwrap();

        // UTF-8 이모지
        file.write_all("이모지: 😀 🎉 🚀\n".as_bytes()).unwrap();

        // NULL 바이트가 포함된 데이터
        file.write_all(b"\x00\x00NULL\x00\x00BYTES\x00\x00")
            .unwrap();

        file_path
    }

    /// 특정 패턴이 경계에 걸쳐있는 테스트 파일 생성
    pub fn create_boundary_test_file() -> PathBuf {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("boundary_test.bin");
        let mut file = File::create(&file_path).unwrap();

        // 4096 바이트 버퍼 크기를 고려한 경계 테스트
        let mut data = vec![0x41u8; 4094]; // 'A' * 4094

        // 버퍼 경계에 걸쳐있는 패턴
        data.extend_from_slice(b"\xAA\xBB"); // 4094-4095 위치
        file.write_all(&data).unwrap();
        file.write_all(b"\xCC\xDD\xEE").unwrap(); // 4096-4098 위치 (다음 버퍼)

        // 더 많은 데이터
        file.write_all(&vec![0x42u8; 1000]).unwrap(); // 'B' * 1000

        file_path
    }

    /// 압축된 형태의 반복 패턴 파일 생성
    pub fn create_repetitive_pattern_file() -> PathBuf {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("repetitive.bin");
        let mut file = File::create(&file_path).unwrap();

        // 같은 패턴이 여러 번 반복
        let pattern = b"\x12\x34\x56\x78";
        for _ in 0..100 {
            file.write_all(pattern).unwrap();
            file.write_all(b"SEPARATOR").unwrap();
        }

        file_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_create_cctv_file() {
        let file_path = TestDataGenerator::create_cctv_test_file();
        assert!(file_path.exists());

        let content = fs::read(&file_path).unwrap();
        // H.264 SPS 시그니처 확인
        assert!(content.windows(5).any(|w| w == b"\x00\x00\x00\x01\x67"));

        fs::remove_file(file_path).ok();
    }

    #[test]
    fn test_create_large_file() {
        let file_path = TestDataGenerator::create_large_test_file(2); // 2MB
        assert!(file_path.exists());

        let metadata = fs::metadata(&file_path).unwrap();
        assert_eq!(metadata.len(), 2 * 1024 * 1024);

        fs::remove_file(file_path).ok();
    }

    #[test]
    fn test_mixed_encoding_file() {
        let file_path = TestDataGenerator::create_mixed_encoding_file();
        assert!(file_path.exists());

        let content = fs::read(&file_path).unwrap();
        // ASCII 텍스트 확인
        assert!(content.windows(5).any(|w| w == b"Hello"));
        // NULL 바이트 확인
        assert!(content.windows(2).any(|w| w == b"\x00\x00"));

        fs::remove_file(file_path).ok();
    }

    #[test]
    fn test_boundary_file() {
        let file_path = TestDataGenerator::create_boundary_test_file();
        assert!(file_path.exists());

        let content = fs::read(&file_path).unwrap();
        // 경계 패턴 확인
        assert!(content.windows(5).any(|w| w == b"\xAA\xBB\xCC\xDD\xEE"));

        fs::remove_file(file_path).ok();
    }

    #[test]
    fn test_repetitive_pattern_file() {
        let file_path = TestDataGenerator::create_repetitive_pattern_file();
        assert!(file_path.exists());

        let content = fs::read(&file_path).unwrap();
        // 반복 패턴 확인
        let pattern_count = content
            .windows(4)
            .filter(|w| *w == b"\x12\x34\x56\x78")
            .count();
        assert_eq!(pattern_count, 100);

        fs::remove_file(file_path).ok();
    }
}
