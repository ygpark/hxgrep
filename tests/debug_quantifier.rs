use regex::bytes::Regex;

#[test]
fn debug_quantifier_behavior() {
    let test_data = b"\x01\x00\x00\x58\x58\x58\x58test";
    let pattern = r"\x58{2,3}";

    println!("Test data: {:?}", test_data);
    println!("Pattern: {}", pattern);

    let regex = Regex::new(pattern).unwrap();
    let matches: Vec<_> = regex.find_iter(test_data).collect();

    println!("Found {} matches:", matches.len());
    for (i, m) in matches.iter().enumerate() {
        println!(
            "  Match {}: position {}-{}, data: {:?}",
            i + 1,
            m.start(),
            m.end(),
            &test_data[m.start()..m.end()]
        );

        // 16진수로 표시
        let hex_string = test_data[m.start()..m.end()]
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");
        println!("    Hex: {}", hex_string);
    }

    // 예상: 2개의 매치만 있어야 함
    // 1. 위치 3-5: 58 58 (2개)  - 아니면 이것도 안될 수 있음
    // 2. 위치 3-6: 58 58 58 (3개)

    // 실제로는 탐욕적 매칭으로 위치 3-6에서 3개만 매칭될 것 같음
}

#[test]
fn test_different_data() {
    // 더 명확한 테스트 데이터
    let test_data = b"XX\x58\x58YY\x58\x58\x58ZZ\x58\x58\x58\x58WW";
    let pattern = r"\x58{2,3}";

    println!("Test data: {:?}", test_data);
    println!("Pattern: {}", pattern);

    let regex = Regex::new(pattern).unwrap();
    let matches: Vec<_> = regex.find_iter(test_data).collect();

    println!("Found {} matches:", matches.len());
    for (i, m) in matches.iter().enumerate() {
        println!(
            "  Match {}: position {}-{}, length: {}",
            i + 1,
            m.start(),
            m.end(),
            m.end() - m.start()
        );

        let hex_string = test_data[m.start()..m.end()]
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");
        println!("    Hex: {}", hex_string);
    }
}

#[test]
fn test_exact_quantifier() {
    let test_data = b"XX\x58\x58YY\x58\x58\x58ZZ\x58\x58\x58\x58WW";
    let pattern = r"\x58{3}"; // 정확히 3개

    println!("Test data: {:?}", test_data);
    println!("Pattern: {}", pattern);

    let regex = Regex::new(pattern).unwrap();
    let matches: Vec<_> = regex.find_iter(test_data).collect();

    println!("Found {} matches:", matches.len());
    for (i, m) in matches.iter().enumerate() {
        println!(
            "  Match {}: position {}-{}, length: {}",
            i + 1,
            m.start(),
            m.end(),
            m.end() - m.start()
        );

        let hex_string = test_data[m.start()..m.end()]
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");
        println!("    Hex: {}", hex_string);
    }

    // 정확히 3개인 경우만 매칭되어야 함
    // 위치 5-8: 58 58 58 (3개)
    // 위치 9-12의 58 58 58은? 연속된 4개 중 첫 3개가 매칭될 것
}
