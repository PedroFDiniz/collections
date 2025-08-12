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
    fn populates_linked_list() {
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

    #[test]
    fn get_linked_list_element_from_index() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for number in 0..=10 { list.push(number); }
        let zero = 0i32;
        let five = 5i32;
        let ten = 10i32;

        assert_eq!(list.get(10), Some(&ten));
        assert_eq!(list.get(5), Some(&five));
        assert_eq!(list.get(0), Some(&zero));
    }

    #[test]
    fn get_wrong_linked_list_element_from_index() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for number in 0..=10 { list.push(number); }
        let zero = 0i32;
        let five = 5i32;
        let ten = 10i32;

        assert_ne!(list.get(9), Some(&ten));
        assert_ne!(list.get(4), Some(&five));
        assert_ne!(list.get(6), Some(&five));
        assert_ne!(list.get(1), Some(&zero));
    }

    #[test]
    fn remove_given_value_from_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for number in 0..=10 { list.push(number); }
        let zero = 0i32;
        let five = 5i32;
        let ten = 10i32;
        assert_eq!(list.size(), 11);
        assert!(list.contains(&zero));
        assert!(list.contains(&five));
        assert!(list.contains(&ten));
        assert!(list.contains(&4));
        assert!(list.contains(&6));

        assert!(list.remove(&five));
        assert_eq!(list.size(), 10);
        assert!(list.contains(&zero));
        assert!(!list.contains(&five));
        assert!(list.contains(&ten));
        assert!(list.contains(&4));
        assert!(list.contains(&6));
    }

    #[test]
    fn remove_item_that_doesnt_exist_in_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for number in 0..=10 { list.push(number); }
        let eleven = 11i32;

        assert_eq!(list.size(), 11);
        assert!(!list.remove(&eleven));
        assert_eq!(list.size(), 11);
    }

    #[test]
    fn remove_item_from_linked_list_at_index() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for number in 0..=10 { list.push(number); }
        let six = 6i32;
        let seven = 7i32;

        assert_eq!(list.size(), 11);
        assert_eq!(list.remove_at(6), Some(six));
        assert_eq!(list.size(), 10);
        assert_eq!(list.get(6), Some(&seven));
    }

    #[test]
    fn remove_item_at_index_zero_on_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for number in 0..=10 { list.push(number); }
        let zero = 0usize;

        assert_eq!(list.size(), 11);
        assert_eq!(list.remove_at(zero), Some(0));
        assert_eq!(list.size(), 10);
        assert_eq!(list.get(0), Some(&1))
    }

    #[test]
    fn remove_item_at_index_from_empty_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.remove_at(0), None);
    }

    #[test]
    fn remove_item_at_index_higher_than_linked_list_size() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for number in 0..=10 { list.push(number); }

        assert_eq!(list.remove_at(12), None);
        assert_eq!(list.remove_at(11), None);
        assert_eq!(list.remove_at(10), Some(10));
    }
}
