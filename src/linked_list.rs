use crate::collection::Collection;

struct Node<T>{
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn get_head(&self) -> Option<&T> {
        return self.head.as_ref().map(|node| &node.value);
    }

    pub fn push(&mut self, value: T) {
        let new_node: Option<Box<Node<T>>> = Some(Box::new(Node {
            value,
            next: None
        }));
        match self.head {
            None => { self.head = new_node; }
            Some(ref mut node) => {
                let mut current_node = node;
                while let Some(ref mut next_node) = current_node.next {
                    current_node = next_node;
                } current_node.next = new_node;
            }
        } self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T>{
        if self.head.is_none() { return None; }

        let mut current_node = self.head.as_mut().unwrap();
        if current_node.next.is_none() {
            match self.head.take() {
                None => return None,
                Some(node) => {
                    self.size -= 1;
                    return Some(node.value);
                }
            }
        }

        while current_node.next.as_ref().unwrap().next.is_some() {
            current_node = current_node.next.as_mut().unwrap();
        }

        let result = current_node.next.take();
        match result {
            Some(node) => {
                self.size -= 1;
                return Some(node.value);
            }
            None => { return None; }
        }
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    }

    pub fn contains(&self, value: &T) -> bool where T: PartialEq {
        let mut current_node = &self.head;
        while let Some(node) = current_node {
            if node.value == *value { return true; }
            current_node = &node.next;
        } return false;
    }
}

impl<T> Collection<T> for LinkedList<T> {
    fn add(&mut self, value: T) {
        self.push(value);
    }

    fn contains(&self, value: T) -> bool where T: PartialEq {
        return self.contains(&value);
    }

    // TODO implement remove specific element
    fn remove(&mut self, value: T) -> bool {
        return false;
    }

    fn size(&self) -> usize {
        return self.size;
    }
}