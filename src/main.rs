use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

fn untokenize(
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
        let replace_val = hashes.get(hash);
        if replace_val.is_none() {
            continue;
        }
        let replace_str = start.to_owned() + replace_val.unwrap() + end;
        txt = txt.replace(&token, &replace_str);
    }
    txt.to_string()
}

fn tokenize(
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
    let mut t = "This is *emphasized* and so is *this* as well.".to_string();
    let mut foo = tokenize(&mut t, "*", "*");
    let result = untokenize(&mut foo.0, &foo.1, "*", "*");
    println!("{}", result);
}

#[test]
fn test_plain() {
    let mut t = "This is just plain text.".to_string();
    let (result, hashes) = tokenize(&mut t, "<code>", "</code>");
    let r = untokenize(&mut result.clone(), &hashes, "<code>", "</code>");
    assert_eq!(r, t);
}

#[test]
fn tes_thtml() {
    let mut t = "This is <code>inline code</code> and so is <code>this text</code> as well.".to_string();
    let (result, hashes) = tokenize(&mut t, "<code>", "</code>");
    let r = untokenize(&mut result.clone(), &hashes, "<code>", "</code>");
    assert_eq!(r, t);
}

#[test]
fn test_markdown_inline_code() {
    let mut t = "This is `inline code` and so is `this text` as well.".to_string();
    let (result, hashes) = tokenize(&mut t, "`", "`");
    let r = untokenize(&mut result.clone(), &hashes, "`", "`");
    assert_eq!(r, t);
}

#[test]
fn test_markdown_inline_em() {
    let mut t = "This is *emphasized* and so is *this* as well.".to_string();
    let (result, hashes) = tokenize(&mut t, "*", "*");
    let r = untokenize(&mut result.clone(), &hashes, "*", "*");
    assert_eq!(r, t);
}

#[test]
fn test_markdown_code_block() {
    let mut t = r###"
Below is inline code:
```
fn main() {
    let x = 5;
}
```
And this comes after the code block.
"###.to_string();
    let (result, hashes) = tokenize(&mut t, "*", "*");
    let r = untokenize(&mut result.clone(), &hashes, "*", "*");
    assert_eq!(r, t);
}
