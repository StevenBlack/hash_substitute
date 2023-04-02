use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

fn un_hash_tokenize(
    text: &mut String,
    hashes: &HashMap<String, String>,
    start: &str,
    end: &str
) -> String {
    let tokenprefix = "[".repeat(4);
    let tokensuffix = "]".repeat(4);
    let mut txt = text.clone();
    for hash in hashes.keys() {
        let token = format!("{tokenprefix}{hash}{tokensuffix}");
        // let replace_str = format!("{start}{:?}{end}",hashes.iter().find(|(_, v)| **v == *hash).unwrap().0);
        let replace_val = hashes.get(hash);
        if replace_val.is_none() {
            continue;
        }
        let replace_str = start.to_owned() + replace_val.unwrap() + end;
        txt = txt.replace(&token, &replace_str);
    }
    txt.to_string()
}

fn hash_tokenize(
    text: &mut String,
    start: &str,
    end: &str
) -> (String, HashMap<String, String>) {
    let tokenprefix = "[".repeat(4);
    let tokensuffix = "]".repeat(4);
    let mut hashes = HashMap::new();
    let mut result = String::new();
    let mut start_index = 0;
    let mut end_index;
    let mut hasher = DefaultHasher::new();

    while let Some(start_pos) = text[start_index..].find(start) {
        let start_idx = start_pos + start_index;
        let end_pos = text[start_idx + start.len()..].find(end);

        if let Some(end_pos) = end_pos {
            end_index = start_idx + start.len() + end_pos;
            let replace_str = &text[start_idx + start.len()..=end_index-1];
            hasher.write(replace_str.as_bytes());
            let hash = format!("{:x}", hasher.finish());
            hashes.insert(hash.clone(), replace_str.to_string());
            result.push_str(&text[start_index..start_idx]);
            result.push_str(&format!("{tokenprefix}{hash}{tokensuffix}"));
            start_index = end_index + end.len();
        } else {
            break;
        }
    }
    result.push_str(&text[start_index..]);
    (result, hashes)
}

fn main() {
    let mut t = "This is <code>inline code</code> and so is <code>this text</code> as well.".to_string();
    let (result, hashes) = hash_tokenize(&mut t, "<code>", "</code>");
    println!("{}", result);
    println!("{:?}", hashes);

    println!("=====");
    let mut t = "This is *emphasized* and so is *this* as well.".to_string();
    let (result, hashes) = hash_tokenize(&mut t, "*", "*");
    println!("{}", result);
    println!("{:?}", hashes);

    println!("=====");
    let mut t = "This is *emphasized* and so is *this* as well.".to_string();
    let mut foo = hash_tokenize(&mut t, "*", "*");
    let result = un_hash_tokenize(&mut foo.0, &foo.1, "*", "*");
    println!("{}", result);
}
