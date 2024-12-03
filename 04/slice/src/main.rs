fn main() {
    let s = String::from("This is a test!");
    let idx = first_word_idx(&s);
    println!("{}", idx);
    println!("{}", first_word(&s));
    println!("{}", first_word("CompleteWord"));
    println!("{}", first_word("ceci est une phrase"));
}

fn first_word_idx(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s
}