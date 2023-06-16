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

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
            // next: Link::Empty,
        });

        self.head = Link::More(new_node);
    }
}

pub fn box_list() {
    let mut list: List = List::new();
    let els = vec![5, 10, 15, 20, 25, 30];
    for el in els {
        list.push(el);
    }
    println!("{:#?}", list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let result = box_list();
    }
}
