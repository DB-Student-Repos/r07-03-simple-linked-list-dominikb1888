use std::iter::FromIterator;

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>
}

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut current_node = &self.head;
        let mut count = 0;
        while let Some(node) = current_node {
            count += 1;
            current_node = &node.next;
        }
        count
    }

    pub fn push(&mut self, _element: T) {
        let new = Node { element: _element, next: self.head.take() };
        self.head = Some(Box::new(new))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        // Reversing means creating the list again
        let mut reversed = SimpleLinkedList::new();
        while let Some(node) = self.pop() {
            reversed.push(node)
        }
        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for el in _iter {
            list.push(el)
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vector = Vec::new();
        while let Some(node) = _linked_list.pop() {
            vector.push(node)
        }
        vector.reverse();
        vector
    }
}
