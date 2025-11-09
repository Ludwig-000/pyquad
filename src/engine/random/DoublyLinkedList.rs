
use std::collections::LinkedList;

pub struct end_node<T>{
    item: Box<T>,
    outwardConnection: connected_Node,
}
pub struct connected_Node<T>{
    owned_node: end_node<T>,
    prev: Option<Box<connected_Node<T>>>,
    next: Option<Box<connected_Node<T>>>,
}


impl<T> end_node<T>{
   fn new(t: T){
    todo!()
   }
}


impl<T> connected_Node<T>{
    
}