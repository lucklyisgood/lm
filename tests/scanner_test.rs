use lm::scanner::Scanner;


#[test]
fn keyword_test() {
    let mut scan = Scanner::new("let");
    let result = scan.scan_tokens();

    assert!(result.is_ok());

    let tokens = result.unwrap();

    assert_eq!(tokens.len(), 1);
}

#[test]
fn string_test() {
    let mut scan = Scanner::new("\"hello\"");
    let result = scan.scan_tokens();

    assert!(result.is_ok());

    let tokens = result.unwrap();

    assert_eq!(tokens.len(), 1);
}