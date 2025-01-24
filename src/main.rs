use rand::Rng;

fn generate_segment() -> String {
    let mut rng = rand::thread_rng();

    let digits = "0123456789";
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = lowercase.to_uppercase();

    // Create a single string first, then convert to chars
    let all_chars = format!("{}{}{}", digits, lowercase, uppercase);
    let chars: Vec<char> = all_chars.chars().collect();

    let segment_len = 6;
    let mut segment = String::with_capacity(segment_len);

    for _ in 0..segment_len {
        let c = chars[rng.gen_range(0..chars.len())];
        segment.push(c);
    }

    segment
}

fn generate_password() -> String {
    // Format: xXxxx-xxXxx-xxxxxX
    let segment1 = generate_segment();
    let segment2 = generate_segment();
    let segment3 = generate_segment();

    format!("{}-{}-{}", segment1, segment2, segment3)
}

fn main() {
    let password = generate_password();
    println!("{}", password);
}
