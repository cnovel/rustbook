use std::collections::HashMap;
use std::io;

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
    struct CompanyCli {
        data: HashMap<String, Vec<String>>,
    }

    impl CompanyCli {
        fn add_to_department(&mut self, person: &str, department: &str) {
            let v = self.data.get_mut(department);
            if let Some(d) = v {
                d.push(person.to_string());
                return;
            }
            self.data.insert(department.to_string(), vec![person.to_string()]);
        }

        fn print_department(&self, department: &str) {
            let d = self.data.get(department);
            if let Some(v) = d {
                println!("People in {department}: {v:#?}");
                return;
            }
            println!("No one works in {department}!");
        }

        fn print_all(&self) {
            let mut deps: Vec<&String> = self.data.keys().collect();
            deps.sort();
            for d in deps {
                self.print_department(&d);
            }
        }

        fn listen(&mut self) {
            loop {
                println!("Enter cmd (type Exit to quit):");
                let mut cmd = String::new();
                io::stdin()
                    .read_line(&mut cmd)
                    .expect("Failed to read line");
                cmd = cmd.trim().to_string();
                let words: Vec<&str> = cmd.split_whitespace().collect();
                if words.len() == 0 {
                    println!("Eror: cmd is empty.");
                    continue;
                }

                let action = words[0];
                match action {
                    "Add" => self.add_to_department(words[1], words[3]),
                    "Show" => self.print_department(words[1]),
                    "List" => self.print_all(),
                    "Exit" => break,
                    _ => println!("Unknown cmd <{}>", cmd),
                }
            }
        }
    }
    let mut c = CompanyCli {
        data: HashMap::new(),
    };
    c.listen();
}
