pub mod collection;
pub mod linked_list;


#[cfg(test)]
mod tests {
    use super::linked_list::LinkedList;

    #[test]
    fn creates_linked_list() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.size(), 0);
        assert!(list.get_head().is_none());
    }

    #[test]
    fn adds_element_to_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(32);
        let arbitrary_value: i32 = 32;
        assert!(list.contains(&arbitrary_value));
        assert_eq!(list.size(), 1);
        assert_eq!(list.get_head(), Some(&arbitrary_value));
    }

    #[test]
    fn adds_more_than_one_element_to_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for number in 0..=10 {
            list.push(number);
        }
        let zero: i32 = 0;
        assert!(list.contains(&0));
        assert!(list.contains(&5));
        assert!(list.contains(&10));
        assert_eq!(list.get_head(), Some(&zero));
        assert_eq!(list.size(), 11);
    }

    #[test]
    fn removes_element_from_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for number in 0..=10 {
            list.push(number);
        }
        let ten: i32 = 10i32;
        assert!(list.contains(&ten));
        assert_eq!(list.size(), 11);
        assert_eq!(list.pop(), Some(ten));
        assert!(!list.contains(&ten));
        assert_eq!(list.size(), 10);
    }

    #[test]
    fn clears_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for number in 0..=10 { list.push(number); }
        let ten: i32 = 10i32;

        assert!(list.contains(&ten));
        assert_eq!(list.size(), 11);

        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
        assert!(!list.contains(&ten));
        assert_eq!(list.get_head(), None);
    }
}
