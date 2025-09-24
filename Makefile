# Makefile for hxgrep
# 전통적인 make 명령어 지원

.PHONY: all build build-release test test-all clean check fmt clippy dev-test pre-commit ci help

# 기본 타겟
all: build test

# 빌드 관련
build:
	@echo "🔨 디버그 빌드 중..."
	cargo build

build-release:
	@echo "🚀 릴리즈 빌드 중..."
	cargo build --release

# 테스트 관련
test:
	@echo "🧪 테스트 실행 중..."
	cargo test

test-all:
	@echo "🧪 모든 테스트 실행 중..."
	cargo test
	cargo test -- --ignored

# 테스트 관련 (이어서)
test-ignored:
	@echo "🧪 무시된 테스트 실행 중..."
	cargo test -- --ignored

# 개발 도구
clean:
	@echo "🧹 정리 중..."
	cargo clean

check:
	@echo "🔍 문법 검사 중..."
	cargo check

fmt:
	@echo "🎨 코드 포맷팅 중..."
	cargo fmt

clippy:
	@echo "📎 린팅 중..."
	cargo clippy

# 통합 명령어
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
	@echo "  make build         - 디버그 빌드"
	@echo "  make build-release - 릴리즈 빌드"
	@echo ""
	@echo "🧪 테스트:"
	@echo "  make test          - 기본 테스트"
	@echo "  make test-all      - 모든 테스트 (무시된 것 포함)"
	@echo ""
	@echo "🛠  개발:"
	@echo "  make clean         - 빌드 정리"
	@echo "  make check         - 문법 검사"
	@echo "  make fmt           - 코드 포맷팅"
	@echo "  make clippy        - 린팅"
	@echo ""
	@echo "🔄 통합:"
	@echo "  make dev-test      - 개발자 테스트"
	@echo "  make pre-commit    - 커밋 전 검사"
	@echo "  make ci            - CI 파이프라인"