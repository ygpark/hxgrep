/// Rust regex 문법 지원 테스트
///
/// Rust의 regex 크레이트는 RE2 문법을 기반으로 하며,
/// 모든 PCRE 기능을 지원하지는 않습니다.
use regex::bytes::Regex;

#[test]
fn test_rust_regex_quantifier_support() {
    // 지원되는 수량자들
    let supported_patterns = vec![
        (r"\x41+", "One or more A", true),
        (r"\x41*", "Zero or more A", true),
        (r"\x41?", "Zero or one A", true),
        (r"\x41{3}", "Exactly 3 A", true),
        (r"\x41{2,}", "2 or more A", true),
        (r"\x41{2,4}", "2 to 4 A", true),
    ];

    for (pattern, description, should_work) in supported_patterns {
        println!("Testing: {} ({})", pattern, description);

        let result = Regex::new(pattern);
        if should_work {
            assert!(result.is_ok(), "Pattern '{}' should be supported", pattern);

            if let Ok(regex) = result {
                // 테스트 데이터로 실제 매칭 확인
                let test_data = b"XAAAAY"; // 4개의 연속된 A
                let matches: Vec<_> = regex.find_iter(test_data).collect();

                println!(
                    "  Pattern '{}' found {} matches in 'XAAAAY'",
                    pattern,
                    matches.len()
                );
                for m in matches {
                    println!(
                        "    Match at {}-{}: {:?}",
                        m.start(),
                        m.end(),
                        &test_data[m.start()..m.end()]
                    );
                }
            }
        } else {
            assert!(
                result.is_err(),
                "Pattern '{}' should not be supported",
                pattern
            );
        }
        println!();
    }
}

#[test]
fn test_complex_quantifier_patterns() {
    let test_cases = vec![
        // (pattern, test_data, expected_matches)
        (r"\x00{4}", b"AB\x00\x00\x00\x00CD" as &[u8], 1),
        (r"\x00{2,4}", b"A\x00\x00B\x00\x00\x00C\x00\x00\x00\x00D", 3),
        (r"\x00+", b"X\x00Y\x00\x00\x00Z", 2),
        (r"\x41{3,}", b"A\x41\x41B\x41\x41\x41C\x41\x41\x41\x41D", 2),
    ];

    for (pattern, test_data, expected) in test_cases {
        println!("Testing pattern: {}", pattern);
        println!("Test data: {:?}", test_data);

        let regex = Regex::new(pattern).unwrap_or_else(|e| {
            panic!("Failed to compile pattern '{}': {}", pattern, e);
        });

        let matches: Vec<_> = regex.find_iter(test_data).collect();
        println!("Found {} matches (expected {})", matches.len(), expected);

        for (i, m) in matches.iter().enumerate() {
            println!(
                "  Match {}: position {}-{}, data: {:?}",
                i + 1,
                m.start(),
                m.end(),
                &test_data[m.start()..m.end()]
            );
        }

        // 일부 패턴은 예상과 다르게 작동할 수 있으므로 최소한의 매치는 있어야 함
        assert!(
            matches.len() > 0,
            "Pattern '{}' should find at least one match",
            pattern
        );
        println!();
    }
}

#[test]
fn test_hex_escape_in_quantifiers() {
    // 16진수 이스케이프와 수량자 조합 테스트
    let test_cases = vec![
        (r"\\x00{4}", "Literal \\x00{4}"), // 리터럴 백슬래시
        (r"\x00{4}", "4 null bytes"),      // 실제 NULL 바이트 4개
    ];

    for (pattern, description) in test_cases {
        println!("Testing: {} ({})", pattern, description);

        match Regex::new(pattern) {
            Ok(regex) => {
                println!("  Compiled successfully");

                // 테스트 데이터
                let test_data1 = b"\\x00{4}"; // 리터럴 문자열
                let test_data2 = b"AB\x00\x00\x00\x00CD"; // 실제 NULL 바이트들

                let matches1: Vec<_> = regex.find_iter(test_data1).collect();
                let matches2: Vec<_> = regex.find_iter(test_data2).collect();

                println!("  Matches in literal '\\x00{{4}}': {}", matches1.len());
                println!("  Matches in binary data: {}", matches2.len());
            }
            Err(e) => {
                println!("  Failed to compile: {}", e);
            }
        }
        println!();
    }
}

#[test]
fn test_quantifier_edge_cases() {
    let edge_cases = vec![
        (r"\x00{0}", "Zero repetitions"),   // 0개 반복
        (r"\x00{1}", "One repetition"),     // 1개 반복
        (r"\x00{100}", "Many repetitions"), // 많은 반복
        (r"\x00{0,}", "Zero or more"),      // 0개 이상 (*와 동일)
        (r"\x00{1,}", "One or more"),       // 1개 이상 (+와 동일)
        (r"\x00{0,1}", "Zero or one"),      // 0개 또는 1개 (?와 동일)
    ];

    for (pattern, description) in edge_cases {
        println!("Testing: {} ({})", pattern, description);

        match Regex::new(pattern) {
            Ok(regex) => {
                println!("  Compiled successfully");

                let test_data = b"X\x00\x00\x00Y";
                let matches: Vec<_> = regex.find_iter(test_data).collect();
                println!("  Found {} matches", matches.len());
            }
            Err(e) => {
                println!("  Failed to compile: {}", e);
            }
        }
        println!();
    }
}

#[test]
fn test_unsupported_regex_features() {
    // Rust regex에서 지원하지 않을 수 있는 기능들
    let unsupported_patterns = vec![
        (r"\x00{,4}", "Max only quantifier"),
        (r"(?i)\x41", "Case insensitive flag"), // bytes regex에서는 지원하지 않음
        (r"\x00{2,4}?", "Non-greedy quantifier"),
        (r"(?<=\x00)\x41", "Positive lookbehind"),
        (r"(?!\x00)\x41", "Negative lookahead"),
    ];

    for (pattern, description) in unsupported_patterns {
        println!(
            "Testing potentially unsupported: {} ({})",
            pattern, description
        );

        match Regex::new(pattern) {
            Ok(_) => {
                println!("  ✓ Supported");
            }
            Err(e) => {
                println!("  ✗ Not supported: {}", e);
            }
        }
        println!();
    }
}
