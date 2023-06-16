#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn hello(self, arg2: &str) -> String {
        "Say hello to ".to_owned() + arg2 + "!"
    }
}

pub fn box_list() {
    let list: List = List::new();
    println!("{:?}", list);
    let list = List::new();
    let hello = list.hello("Ismael");
    println!("{}", hello);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let result = box_list();
    }
}
