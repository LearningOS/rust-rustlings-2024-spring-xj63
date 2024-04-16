/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

#[derive(Debug)]
struct Link<T>(Option<Box<Node<T>>>);

impl<T> Default for Link<T> {
    fn default() -> Self {
        Link(None)
    }
}

impl<T> Link<T> {
    fn new(v: T) -> Self {
        Link(Some(Box::new(Node::new(v))))
    }

    fn raw_ptr(&self) -> Option<*mut Node<T>> {
        match self.0 {
            Some(ref node) => Some(Box::as_ref(node) as *const Node<T> as *mut Node<T>),
            None => None,
        }
    }

    fn replace(&mut self, other: Link<T>) -> Link<T> {
        let origin = self.0.take();
        self.0 = other.0;
        Link(origin)
    }
}

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: Link::default(),
        }
    }

    fn replace_next(&mut self, next: Link<T>) -> Link<T> {
        self.next.replace(next)
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: usize,
    head: Link<T>,
    tail: Option<*mut Node<T>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self {
            length: 0,
            head: Link(None),
            tail: None,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn only_one(&mut self, one: T) {
        let link = Link::new(one);
        self.length = 1;
        self.tail = link.raw_ptr();
        self.head = link;
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn push_back(&mut self, obj: T) {
        if self.is_empty() {
            self.only_one(obj);
            return;
        }

        self.length += 1;
        let link = Link::new(obj);
        let tail = self.tail.replace(link.raw_ptr().unwrap()).unwrap();
        unsafe {
            tail.as_mut().unwrap().next = link;
        }
    }

    pub fn get(&mut self, index: usize) -> Option<&T> {
        Self::get_ith_node(&self.head, index)
    }

    fn get_ith_node(link: &Link<T>, index: usize) -> Option<&T> {
        match &link.0 {
            None => None,
            Some(node) => match index {
                0 => Some(&node.val),
                _ => Self::get_ith_node(&node.next, index - 1),
            },
        }
    }

    fn add(&mut self, t: T) {
        self.push_back(t)
    }
}

impl<T> LinkedList<T>
where
    T: std::cmp::PartialOrd,
{
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        let mut a = list_a.into_iter();
        let mut b = list_b.into_iter();
        let mut ret = LinkedList::new();

        let mut an = match a.next() {
            Some(n) => n,
            None => {
                b.for_each(|v| ret.add(v));
                return ret;
            }
        };
        let mut bn = match b.next() {
            Some(n) => n,
            None => {
                a.for_each(|v| ret.add(v));
                return ret;
            }
        };

        loop {
            if an > bn {
                ret.add(bn);
                bn = match b.next() {
                    Some(n) => n,
                    None => {
                        ret.add(an);
                        break;
                    }
                }
            } else {
                ret.add(an);
                an = match a.next() {
                    Some(n) => n,
                    None => {
                        ret.add(bn);
                        break;
                    }
                }
            }
        }

        a.for_each(|v| ret.add(v));
        b.for_each(|v| ret.add(v));
        ret
    }
}

#[derive(Debug)]
struct LinkedListIter<T> {
    link: Link<T>,
}
impl<T> Iterator for LinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.link.0.take() {
            None => None,
            Some(node) => {
                self.link = node.next;
                Some(node.val)
            }
        }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;

    type IntoIter = LinkedListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter { link: self.head }
    }
}

fn main() {
    let mut list: LinkedList<usize> = LinkedList::new();
    (0..1000).for_each(|e| list.push_back(e));
    println!("{list}");
}

use std::fmt::{self, Display, Formatter};

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.head.0 {
            Some(node) => write!(f, "{}", node),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.next.0 {
            Some(node) => write!(f, "{}, {}", self.val, node),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i).unwrap());
        }
    }
}
