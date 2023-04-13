use std::{cell::RefCell, rc::Rc};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

#[derive(Debug, Clone)]
pub struct TransactionLog {
    head: Link,
    tail: Link,
    pub length: u64,
}

pub struct ListIterator {
    current: Link,
}

impl ListIterator {
    fn new(start_at: Link) -> Self {
        ListIterator { current: start_at }
    }
}

impl Iterator for ListIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let current = self.current.clone();
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            }
            None => None,
        };
        result
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            }
            None => None,
        };
        result
    }
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        }
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None, prev: None }))
    }

}


fn main(){
    let mut log = TransactionLog::new_empty();
    log.append("first".to_string());
    log.append("second".to_string());
    log.append("third".to_string());
    log.append("fourth".to_string());
    log.append("fifth".to_string());
    log.append("sixth".to_string());
    log.append("seventh".to_string());
    log.append("eighth".to_string());
    log.append("ninth".to_string());
    log.append("tenth".to_string());

    let mut iter = ListIterator::new(log.head.clone());
    while let Some(value) = iter.next() {
        println!("{}", value);
    }

}
