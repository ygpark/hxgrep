use regex::bytes::Regex;

fn main() {
    let test_data = b"\x01\x00\x00\x58\x58\x58\x58test";
    let pattern = r"\x58{2,3}";

    println!("Test data: {:?}", test_data);
    println!("Pattern: {}", pattern);

    let regex = Regex::new(pattern).unwrap();
    let matches: Vec<_> = regex.find_iter(test_data).collect();

    println!("Found {} matches:", matches.len());
    for (i, m) in matches.iter().enumerate() {
        println!("  Match {}: position {}-{}, data: {:?}",
                i+1, m.start(), m.end(), &test_data[m.start()..m.end()]);

        // 16진수로 표시
        let hex_string = test_data[m.start()..m.end()].iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");
        println!("    Hex: {}", hex_string);
    }
}