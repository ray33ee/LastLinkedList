
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use std::time::{Instant};
use std::collections::LinkedList;

type NodeRef<T> = Option<Rc<RefCell<Node<T>>>>;
type NodeWeak<T> = Option<Weak<RefCell<Node<T>>>>;

struct Node<T>
{
    _next: NodeRef<T>,
    _data: T
}

impl<T> Node<T>
{
    fn new(node: NodeRef<T>, data: T) -> Self {
        Node {
            _next: node,
            _data: data
        }
    }
}

struct Iterator<T>
{
    _reference: NodeRef<T>
}

impl<T> std::iter::Iterator for Iterator<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self._reference {
            Some(ref current_node) => {

                let tmp = current_node.clone();

                let next_node = current_node.borrow()._next.clone();

                self._reference = next_node;

                let count = Rc::strong_count(&tmp);

                match Rc::try_unwrap(tmp) {
                    Ok(ref_cell) => {
                        Some(ref_cell.into_inner()._data)
                    }
                    Err(_) => {
                        panic!("Critical logic error. Tried to unwrap RC pointer with {} strong references remaining.", count);
                        //None
                    }
                }

                //Some(current_data)
            }
            None => None
        }
    }
}

struct LastLinkedList<T>
{
    _head: NodeRef<T>,
    _tail: NodeWeak<T>
}

impl<T> LastLinkedList<T>
{
    fn new() -> Self {
        LastLinkedList {
            _head: None,
            _tail: None
        }
    }

    fn append(&mut self, data: T) {

        let ref_node = Rc::new(RefCell::new(Node::new(None, data)));

        match self._tail.take() {
            Some(ref old_tail) => {
                let strong_tail = old_tail.upgrade().unwrap();
                strong_tail.borrow_mut()._next = Some(ref_node.clone());
            }
            None => {
                self._head = Some(ref_node.clone());
            }
        }

        self._tail = Some(Rc::downgrade(&ref_node));
    }

    fn push_back(&mut self, data: T) {
        self.append(data);
    }

    fn prepend(&mut self, data: T) {

        let ref_node = Rc::new(RefCell::new(Node::new(None, data)));

        match self._head.take() {
            Some(ref old_head) => {
                ref_node.borrow_mut()._next = Some(old_head.clone());
            }
            None => {
                self._tail = Some(Rc::downgrade(&ref_node));
            }
        }

        self._head = Some(ref_node.clone());

    }

}

impl<T> std::iter::IntoIterator for LastLinkedList<T>
{
    type Item = T;
    type IntoIter = Iterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Iterator {
            _reference: self._head.clone()
        }
    }
}

impl<T> std::ops::Drop for LastLinkedList<T> {
    fn drop(&mut self) {
        let mut node = self._head.clone();

        self._head = None;
        self._tail = None;

        loop {
            match node.take() {
                Some(raw_node) => {
                    let next_node = raw_node.borrow()._next.clone();

                    match Rc::try_unwrap(raw_node) {
                        Ok(ref_cell) => {
                            ref_cell.into_inner();
                        }
                        Err(_) => {
                            // If we find a node with more than one strong reference, we should stop immediately
                            break
                        }
                    }
                    node = next_node;
                }
                None => {
                    break
                }

            }

        }
    }
}

impl<T> std::ops::Drop for Iterator<T> {
    fn drop(&mut self) {
        let mut node = self._reference.clone();

        self._reference = None;

        loop {
            match node.take() {
                Some(raw_node) => {
                    let next_node = raw_node.borrow()._next.clone();

                    match Rc::try_unwrap(raw_node) {
                        Ok(ref_cell) => {
                            ref_cell.into_inner(); //Calling this function will destroy the cell then return the inner value. SInce we dont assign it to anything, it is destroyed
                        }
                        Err(_) => {
                            // If we find a node with more than one strong reference, we should stop immediately
                            break
                        }
                    }
                    node = next_node;
                }
                None => {
                    break
                }

            }

        }
    }
}

fn main() {
    println!("Hello, world!");

    let array_size = 100000;

    let mut lll = LastLinkedList::<i32>::new();

    //let mut lll = LinkedList::new();


    let start = Instant::now();

    for i in 0..array_size {
        lll.push_back(i);
    }

    let duration = start.elapsed();
    println!("Elapsed: {:?}", duration);

    /*for i in lll {
        println!("{}", i);
    }*/

}
