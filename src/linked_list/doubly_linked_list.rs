use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Eq + PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Node<T> {
    pub fn new(
        value: T,
        next: Option<Rc<RefCell<Node<T>>>>,
        prev: Option<Rc<RefCell<Node<T>>>>,
    ) -> Node<T> {
        Self { value, next, prev }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn add_head(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }));

        match self.head.take() {
            Some(old_head) => {
                new_node.borrow_mut().next = Some(Rc::clone(&old_head));
                old_head.borrow_mut().prev = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_singly_linked_list() {
        let mut node = LinkedList::<u32>::new();
        assert!(node.is_empty());

        node.add_head(1);
        node.add_head(2);
        node.add_head(3);

        // 3 -> 2 -> 1
        assert_eq!(node.head.as_ref().unwrap().borrow().value, 3);
        assert_eq!(node.head.as_ref().unwrap().borrow().prev, None);
        assert_eq!(
            node.head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value,
            2
        );
        assert_eq!(
            node.head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value,
            1
        );
        assert_eq!(
            node.head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .next,
            None
        );

        // 1 -> 2 -> 3
        assert_eq!(
            node.head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .prev
                .as_ref()
                .unwrap()
                .borrow()
                .value,
            2
        );
        assert_eq!(
            node.head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .prev
                .as_ref()
                .unwrap()
                .borrow()
                .prev
                .as_ref()
                .unwrap()
                .borrow()
                .value,
            3
        );
    }
}
