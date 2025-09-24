# hxgrep

바이너리 파일에서 정규표현식 패턴을 검색하는 Rust 기반 도구입니다.

## 기능

- 바이너리 파일을 16진수로 표시
- 정규표현식을 사용한 바이너리 패턴 검색
- `\xHH` 형식의 16진수 패턴 지원
- 유연한 출력 포맷 (오프셋, 구분자, 라인 폭 조정)
- 대용량 파일 효율적 처리

## 빌드 및 설치

```bash
# 디버그 빌드
cargo build

# 릴리즈 빌드 (최적화)
cargo build --release

# 빌드 후 바로 실행
cargo run -- [옵션] <파일경로>
```

## 사용법

### 기본 사용법

```bash
# 파일 내용을 16진수로 출력
./target/release/hxgrep file.bin

# 정규표현식으로 패턴 검색
./target/release/hxgrep file.bin -e "\x00\x00\x00\x01\x67"

# 한 줄에 8바이트씩 출력
./target/release/hxgrep file.bin -w 8

# 처음 10줄만 출력
./target/release/hxgrep file.bin -n 10

# 오프셋 숨기기
./target/release/hxgrep file.bin --hideoffset

# 구분자 변경
./target/release/hxgrep file.bin -t "-"
```

### 옵션 설명

- `-e, --regex <PATTERN>`: 정규표현식 패턴
- `-w, --width <N>`: 한 줄에 표시할 바이트 개수 (기본값: 16)
- `-n, --line <N>`: 출력할 라인 수 (0: 무제한)
- `-s, --position <N>`: 시작 위치 (바이트 단위)
- `-t, --separator <STR>`: 바이트 문자열 분리 기호
- `--hideoffset`: 오프셋 출력 안함
- `-h, --help`: 도움말 표시
- `-V, --version`: 버전 정보 표시

### 사용 예제

#### 정규표현식 수량자 사용
```bash
# 정확히 4개의 NULL 바이트
./target/release/hxgrep file.bin -e "\x00{4}"

# 2-4개의 NULL 바이트
./target/release/hxgrep file.bin -e "\x00{2,4}"

# 1개 이상의 0xFF 바이트
./target/release/hxgrep file.bin -e "\xFF+"

# 0개 이상의 0x20 바이트 (공백)
./target/release/hxgrep file.bin -e "\x20*"

# 선택적인 바이트 (0개 또는 1개)
./target/release/hxgrep file.bin -e "\x0D\x0A?"
```

#### H.264 NAL Unit 검색
```bash
# SPS (Sequence Parameter Set) 검색
./target/release/hxgrep video.mp4 -e "\x00\x00\x00\x01\x67"

# PPS (Picture Parameter Set) 검색
./target/release/hxgrep video.mp4 -e "\x00\x00\x00\x01\x68"

# IDR 프레임 검색
./target/release/hxgrep video.mp4 -e "\x00\x00\x00\x01\x65"

# 유연한 NAL unit 시작 코드 (2-3개 NULL 바이트)
./target/release/hxgrep video.mp4 -e "\x00{2,3}\x01"
```

#### 실행 파일 분석
```bash
# PE 헤더 검색
./target/release/hxgrep program.exe -e "\x4D\x5A" -w 32

# ELF 헤더 검색
./target/release/hxgrep program -e "\x7F\x45\x4C\x46" -w 32
```

#### 데이터베이스 파일 분석
```bash
# SQLite 시그니처 검색
./target/release/hxgrep database.db -e "\x53\x51\x4C\x69\x74\x65"
```

## 테스트

### 기본 테스트 실행
```bash
# 모든 테스트 실행
./run_tests.sh

# 또는 개별 실행
cargo test                           # 유닛 테스트
cargo test --test integration_test   # 통합 테스트
cargo test --test test_data_generator # 테스트 데이터 생성기 테스트
cargo test --test regex_quantifiers_test # 정규표현식 수량자 테스트
cargo test --test regex_syntax_test  # 정규표현식 문법 지원 테스트
```


### 테스트 커버리지
```bash
# tarpaulin 설치
cargo install cargo-tarpaulin

# HTML 커버리지 리포트 생성
cargo tarpaulin --out Html
```

## 테스트 구성

### 유닛 테스트
- `parse_hex_pattern()` 함수 테스트
- `escape_bytes_for_regex()` 함수 테스트
- 16진수 포맷팅 테스트
- 정규표현식 매칭 테스트

### 통합 테스트
- 파일 I/O 테스트
- 명령줄 옵션 테스트
- 다양한 파일 형식 테스트
- 에러 처리 테스트

### 성능 테스트
- 대용량 파일 처리 성능
- 메모리 사용량 측정
- 복잡한 정규표현식 성능
- 다수 매칭 처리 성능

## 성능 특성

- **메모리 효율적**: 스트리밍 방식으로 대용량 파일 처리
- **빠른 검색**: Rust regex 엔진 활용
- **안정적**: 버퍼 오버플로우 방지
- **유연한 패턴**: 16진수 및 일반 정규표현식 지원

## 원본 프로젝트와의 차이점

C# 원본과 비교하여 다음 기능은 제외되었습니다:
- 물리 디스크 액세스 (`\.\PHYSICALDRIVE*`)
- EWF (E01) 포렌식 이미지 지원
- libewf 의존성

## 라이선스

원본 프로젝트와 동일한 라이선스를 따릅니다.

## 기여

버그 리포트나 기능 요청은 이슈로 등록해 주세요.