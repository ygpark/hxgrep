# justfile - 명령어 단축 스크립트
# 설치: cargo install just
# 사용: just <command>

# 기본 명령어들
build:
    cargo build

build-release:
    cargo build --release

test:
    cargo test

test-all:
    cargo test
    cargo test -- --ignored

# 테스트 관련
test-ignored:
    cargo test -- --ignored

# 개발 관련
clean:
    cargo clean

check:
    cargo check

fmt:
    cargo fmt

clippy:
    cargo clippy

# 통합 명령어들
dev-test: test clippy
    @echo "✅ 개발자 테스트 완료!"

pre-commit: fmt clippy test
    @echo "✅ 커밋 전 검사 완료!"

ci: clean build test-all
    @echo "✅ CI 파이프라인 완료!"

# 도움말
help:
    @echo "📋 사용 가능한 명령어들:"
    @echo ""
    @echo "🔨 빌드:"
    @echo "  build         - 디버그 빌드"
    @echo "  build-release - 릴리즈 빌드"
    @echo ""
    @echo "🧪 테스트:"
    @echo "  test          - 기본 테스트"
    @echo "  test-all      - 모든 테스트 (무시된 것 포함)"
    @echo ""
    @echo "🛠  개발:"
    @echo "  clean         - 빌드 정리"
    @echo "  check         - 문법 검사"
    @echo "  fmt           - 코드 포맷팅"
    @echo "  clippy        - 린팅"
    @echo ""
    @echo "🔄 통합:"
    @echo "  dev-test      - 개발자 테스트"
    @echo "  pre-commit    - 커밋 전 검사"
    @echo "  ci            - CI 파이프라인"

# 기본 명령어 설정
default: help