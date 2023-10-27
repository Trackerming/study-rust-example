use std::mem;

pub struct List<T: Copy> {
    head: Link<T>,
}

enum Link<T: Copy> {
    Empty,
    More(Box<Node<T>>),
}

struct Node<T: Copy> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl<T: Copy> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut box_node) = cur_link {
            cur_link = mem::replace(&mut box_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::List;

    #[test]
    fn basics() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.pop(), None);
        // push元素
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn long_list() {
        let mut list = List::new();
        for i in 0..100000 {
            list.push(i);
        }
        drop(list);
    }
}
