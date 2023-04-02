use std::collections::HashMap;
use md5;

fn hash_tokenize(text: &mut str, start: &str, end: &str) -> (String, HashMap<String, String>) {
    let mut hashes = HashMap::new();
    let mut result = String::new();
    let mut start_index = 0;
    let mut end_index = 0;

    while let Some(start_pos) = text[start_index..].find(start) {
        let start_idx = start_pos + start_index;
        let end_pos = text[start_idx + start.len()..].find(end);

        if let Some(end_pos) = end_pos {
            end_index = start_idx + start.len() + end_pos;
            let replace_str = &text[start_idx + start.len()..=end_index-1];
            let hash = format!("{:x}", md5::compute(replace_str));
            hashes.insert(replace_str.to_string(), hash.clone());
            result.push_str(&text[start_index..start_idx]);
            result.push_str(&format!("[[[{}]]]", hash));
            start_index = end_index + end.len();
        } else {
            break;
        }
    }

    result.push_str(&text[start_index..]);

    (result, hashes)
}

fn main() {
    let mut text = "This is <code>inline code</code> and also <code>this text</code> so they should <code>all</code> be tokenized.".to_string();
    let (result, hashes) = hash_tokenize(&mut text, "<code>", "</code>");
    println!("{}", result);
    println!("{:?}", hashes);
    println!("=====");
    let mut text = "This is *emphasized* and also *this text* so they should *all* be tokenized.".to_string();
    let (result, hashes) = hash_tokenize(&mut text, "*", "*");
    println!("{}", result);
    println!("{:?}", hashes);
}