pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

pub fn box_list() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let result = box_list();
    }
}
