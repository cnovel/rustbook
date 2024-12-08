use std::collections::HashMap;

fn convert_to_pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    if word.len() == 0 {
        return word.to_string();
    }
    if vowels.contains(&word.chars().next().unwrap()) {
        let s = format!("{word}-hay");
        return s;
    }
    let mut s = word[1..].to_string();
    let c = &word.chars().next().unwrap();
    s = format!("{s}-{c}ay");

    return s;
}

fn main() {
    // 1
    let mut v = vec![0, 5, 5, 8, 4, 3, 5, 3, 10, 15, 12, 22, 18, 55];
    v.sort();
    let mid = v.len() / 2;
    println!("Median is {}", v[mid]);

    let mut count_i = HashMap::new();

    for i in v {

        let c = count_i.entry(i).or_insert(0);
        *c += 1;
    }

    let mut max = 0;
    let mut best = 0;
    for (i, c) in count_i {
        if c > max {
            best = i;
            max = c;
        }
    }
    println!("Mode is {best}");

    // 2
    let words = vec!["first", "apple", "test", "open", "check"];
    for word in &words {
        println!("{} translated to {}", word, convert_to_pig_latin(&word));
    }

    // 3
    
}
