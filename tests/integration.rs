use passgen::*;

#[test]
fn test_segment_length() {
    let segment = generate_segment();
    assert_eq!(segment.len(), SEGMENT_LENGTH);
}

#[test]
fn test_password_format() {
    let password = generate_password();
    
    // Check total length: 6 + 6 + 6 + 2 separators
    assert_eq!(password.len(), SEGMENT_LENGTH * 3 + 2);
    
    // Check separators
    assert!(password.chars().nth(SEGMENT_LENGTH).unwrap() == SEGMENT_SEPARATOR);
    assert!(password.chars().nth(SEGMENT_LENGTH * 2 + 1).unwrap() == SEGMENT_SEPARATOR);
}

#[test]
fn test_character_set() {
    let segment = generate_segment();
    assert!(segment.chars().all(|c| c.is_ascii_digit() || c.is_ascii_alphabetic()));
} 
