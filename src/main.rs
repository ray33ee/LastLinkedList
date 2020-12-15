
use std::rc::Rc;
use std::cell::RefCell;

type NodeRef<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T>
    where T: Clone
{
    _next: NodeRef<T>,
    _data: T
}

impl<T> Node<T>
    where T: Clone
{
    fn new(node: NodeRef<T>, data: T) -> Self {
        Node {
            _next: node,
            _data: data
        }
    }
}

struct Iterator<T>
    where T: Clone
{
    _reference: NodeRef<T>
}

impl<T> std::iter::Iterator for Iterator<T>
    where T: Clone
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self._reference {
            Some(ref current_node) => {

                let tmp = current_node.clone();

                let next_node = current_node.borrow_mut()._next.clone();

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
    where T: Clone
{
    _head: NodeRef<T>,
    _tail: NodeRef<T>
}

impl<T> LastLinkedList<T>
    where T: Clone
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
                old_tail.borrow_mut()._next = Some(ref_node.clone());
            }
            None => {
                self._head = Some(ref_node.clone());
            }
        }

        self._tail = Some(ref_node.clone());
    }

    fn prepend(&mut self, data: T) {

        let ref_node = Rc::new(RefCell::new(Node::new(None, data)));

        match self._head.take() {
            Some(ref old_head) => {
                ref_node.borrow_mut()._next = Some(old_head.clone());
            }
            None => {
                self._head = Some(ref_node.clone());
            }
        }

        self._head = Some(ref_node.clone());

    }



    /*fn print(&mut self) {
        let mut it = self._head.clone().unwrap();
        loop {

            println!("{}", it.borrow_mut()._data);

            let bor = it.borrow_mut()._next.clone();

            if bor.is_none() {
                break;
            }

            it = bor.unwrap();
        }
    }*/

}

impl<T> std::iter::IntoIterator for LastLinkedList<T>
    where T: Clone
{
    type Item = T;
    type IntoIter = Iterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Iterator {
            _reference: self._head
        }

    }
}

fn main() {
    println!("Hello, world!");

    let mut lll = LastLinkedList::<i32>::new();

    lll.append(33);
    lll.append(3);
    lll.append(11);

    lll.prepend(22);

    for i in lll {
        println!("{}", i);
    }

    let mut ve = Vec::<i32>::new();

    for i in ve {

    }


}
