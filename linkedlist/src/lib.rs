struct LinkedList<'a, T: 'a> {
    value: T,
    next: Option<&'a LinkedList<'a, T>>,
}

impl<'a, T> LinkedList<'a, T> {
    fn new(value: T) -> LinkedList<'a, T> {
        LinkedList { value, next: None }
    }

    fn push(value: T) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_list() {
        let list = LinkedList::new(1);

        assert_eq!(list.value, 1);
        assert!(list.next.is_none());
    }

    #[test]
    fn should_push() {
        let list = LinkedList::new(1);

        list.push(2);

        assert_eq!(list.value, 1);
        assert_eq!(list.next.unwrap().value, 2);
    }
}
