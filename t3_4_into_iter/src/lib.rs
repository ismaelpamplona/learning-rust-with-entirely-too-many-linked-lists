#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        let mut cloned_list = List::new();
        let mut current = &self.head;
        let mut prev_cloned_node: Option<&mut Box<Node<T>>> = None;

        while let Some(node) = current {
            let cloned_node = Box::new(Node {
                elem: node.elem.clone(),
                next: None,
            });

            if let Some(prev_node) = prev_cloned_node {
                prev_node.next = Some(cloned_node);
                prev_cloned_node = prev_node.next.as_mut().map(|n| &mut *n);
            } else {
                cloned_list.head = Some(cloned_node);
                prev_cloned_node = cloned_list.head.as_mut().map(|n| &mut *n);
            }

            current = &node.next;
        }

        cloned_list
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
#[derive(Debug)]
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

#[test]
fn into_iter() {
    let mut list = List::new();
    let els = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for el in &els {
        list.push(*el);
    }

    let mut iter = list.clone().into_iter();
    println!("##### First iter");
    println!("{:?}", iter);
    for el in els.iter().rev().cloned() {
        assert_eq!(iter.next(), Some(el));
    }
    println!("##### Second iter");
    println!("{:?}", iter);
    println!("##### List");
    println!("{:?}", list);
}
