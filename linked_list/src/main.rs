use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
struct Node {
    value: String,
    next: Option<Rc<RefCell<Node>>>,
}

struct TransactionLog {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    pub length: u32,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
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

fn main() {
    let mut log = TransactionLog::new_empty();
    log.append("a".to_string());
    log.append("b".to_string());
    log.append("c".to_string());
    log.append("d".to_string());
    log.append("e".to_string());
    log.append("f".to_string());
    log.append("g".to_string());
    log.append("h".to_string());
    log.append("i".to_string());
    log.append("j".to_string());
    log.append("k".to_string());
    log.append("l".to_string());
    log.append("m".to_string());
    log.append("n".to_string());
    log.append("o".to_string());
    log.append("p".to_string());
    log.append("q".to_string());
    log.append("r".to_string());
    log.append("s".to_string());
    log.append("t".to_string());
    log.append("u".to_string());
    log.append("v".to_string());
    log.append("w".to_string());
    log.append("x".to_string());
    log.append("y".to_string());
    log.append("z".to_string());
    println!("Length: {}", log.length);
    println!("Pop: {}", log.pop().unwrap());
    println!("Pop: {}", log.pop().unwrap());
    println!("Pop: {}", log.pop().unwrap());
}
