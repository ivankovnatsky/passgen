use rand::Rng;

// Constants for better maintainability and configuration
pub const SEGMENT_LENGTH: usize = 6;
pub const SEGMENT_SEPARATOR: char = '-';
pub const UPPERCASE_PROBABILITY: f64 = 0.2;

/// Generates a single segment of the password
pub fn generate_segment() -> String {
    let mut rng = rand::thread_rng();
    
    // Static character sets
    let digits = "0123456789";
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    
    // Pre-allocate with exact capacity
    let mut segment = String::with_capacity(SEGMENT_LENGTH);
    
    // Create character pool (avoid recreating for each segment)
    let chars: Vec<char> = format!("{}{}", digits, lowercase)
        .chars()
        .collect();
    
    for _ in 0..SEGMENT_LENGTH {
        let mut c = chars[rng.gen_range(0..chars.len())];
        
        // Apply uppercase with probability for letters
        if c.is_alphabetic() && rng.gen_bool(UPPERCASE_PROBABILITY) {
            c = c.to_ascii_uppercase();
        }
        
        segment.push(c);
    }
    
    segment
}

/// Generates a complete password in the format: xxxxx-xxxxx-xxxxx
/// where x can be any lowercase/uppercase letter or number
pub fn generate_password() -> String {
    let segments = [
        generate_segment(),
        generate_segment(),
        generate_segment(),
    ];
    
    // Join segments with separator
    segments.join(&SEGMENT_SEPARATOR.to_string())
} 
