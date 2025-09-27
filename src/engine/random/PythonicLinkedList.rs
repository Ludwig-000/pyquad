



pub struct P_Linked_List<T> {
    head: Option<Box<Element<T>>>,
}


struct Element<T> {
    item: T,
    next:Option<Box<Element<T>>>,
}

impl<T> P_Linked_List<T> {

    pub fn new() -> P_Linked_List<T> {
        P_Linked_List { head: None }
    }
    
    pub fn add(&mut self, item: T) {
        let new_element = Box::new(Element {
            item,
            next: self.head.take(),
        });

        self.head = Some(new_element);
    }

    pub fn to_linked_list(&mut self)-> Vec<T>{
        let mut vec = Vec::new();
        
        while let Some(mut boxed_node) = self.head.take() {
            vec.push(boxed_node.item);
            self.head = boxed_node.next;
        }
        vec
    }


}