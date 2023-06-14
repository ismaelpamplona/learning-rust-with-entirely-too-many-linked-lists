#[derive(Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

pub fn box_list() -> List<i32> {
    let lkd_l = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", lkd_l);
    lkd_l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let result = box_list();
    }
}
