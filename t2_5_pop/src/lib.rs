#[derive(PartialEq, Debug)]
pub struct List {
    head: Link,
}

#[derive(PartialEq, Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(PartialEq, Debug)]
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

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let mut list_a = List::new();
        let nums = vec![1, 2, 3, 4, 5];
        for n in nums {
            list_a.push(n);
        }
        let popped = list_a.pop();
        println!("{:?}", popped);
        let mut list_b = List::new();
        let nums = vec![1, 2, 3, 4];
        for n in nums {
            list_b.push(n);
        }
        assert_eq!(list_a, list_b);
    }
}
