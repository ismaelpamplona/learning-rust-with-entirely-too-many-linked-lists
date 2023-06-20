use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

pub fn main() {
    println!("Let's code! 🚀 ");

    let string1 = "xyz";
    let result: &str;
    {
        let string2 = "abcd";
        result = longest(string1, string2);
    }

    println!("The longest string is {}", result);

    // ##############

    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence = novel.split('.').next().expect("Could not find");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);

    // ##############

    let s = "Timtones";
    let t = &s[2..5];
    println!("{:?}", t);

    // ##############

    let x: &'static str = "I have a static lifetime and I can live as long as the program duration";

    // ##############

    let father = "Tim Tones";
    let son = "Jay G";
    let ann = "Nice family!";

    longest_with_an_announcement(father, son, ann);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn test_b<'a>() -> &'a String {
//     let result = String::from("test");
//     &result // cannot return reference to local variable `result`
// }

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_00() {
        main()
    }
}
