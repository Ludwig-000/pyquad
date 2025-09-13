//use std::sync::{Mutex, OnceLock};
//use std::mem;
//use crate::engine::structures::*;
//use std::sync::Arc;

////// vector storing the detected collisions
////lazy_static! {
////    static ref OUTPUT: Mutex<Vec<usize>> = Mutex::new(Vec::new());
////}




//static COLLISION_COMMANDS: OnceLock<Mutex<Vec<CollisionCommand>>> = OnceLock::new();

////static PERSISTENT_COLLISION_COMMANDS: OnceLock<Mutex<Vec<CollisionCommand>>> = OnceLock::new();

////static OUTPUT: OnceLock<Mutex<Vec<CollisionCommand>>> = OnceLock::new();





//struct CollisionCommand {
//    pub rect1: Rectangle,
//    pub rect2: Rectangle,
//    pub function: CollisionFunction,
//}

//// Define an enum with predefined function choices
//enum CollisionFunction {
//    FunctionA,
//    FunctionB,
//}


//impl CollisionFunction {
//    fn call(&self, rect1: &Rectangle, rect2: &Rectangle) {
//        match self {
//            CollisionFunction::FunctionA => Self::function_a(rect1, rect2),
//            CollisionFunction::FunctionB => Self::function_b(rect1, rect2),
//        }
//    }

//    fn function_a(rect1: &Rectangle, rect2: &Rectangle) {
//        println!(
//            "Function A executed! Rect1: {:?}, Rect2: {:?}",
//            rect1.x, rect2.x
//        );
//    }

//    fn function_b(rect1: &Rectangle, rect2: &Rectangle) {
//        println!(
//            "Function B executed! Special response for Rect1: {:?}, Rect2: {:?}",
//            rect1.x, rect2.x
//        );
//    }
//}

//struct CollisionReturn {
//    pub rect1: Rectangle,
//    pub rect2: Rectangle,
//    pub result: bool,
//}



////struct CollisionBuffer<'a,'b > {
////    commands: Vec<CollisionCommand<'a,'b >>,
////}



////let mut temp_collisions = take_from_mutex_vec(&COLLISION_COMMANDS);
//fn take_from_mutex_vec<T>(mutex_vec: &Mutex<Vec<T>>) -> Vec<T> {
//    let mut locked_vec = mutex_vec.lock().unwrap();
//    mem::take(&mut *locked_vec) // ✅ Efficiently swaps out the vector
//}


////put_back_into_mutex_vec(&COLLISION_COMMANDS, temp_collisions);
//fn return_to_mutex_vec<T>(mutex_vec: &Mutex<Vec<T>>, new_data: Vec<T>) {
//    let mut locked_vec = mutex_vec.lock().unwrap();
//    *locked_vec = new_data; // ✅ Efficient replacement
//}



//fn add_collision_check<'a>(rect1: &'a Rectangle<'a>, rect2: &'a Rectangle<'a>) {
//    let collision = CollisionCommand { rect1, rect2 };

    
//    let mutex_vec = COLLISION_COMMANDS.get_or_init(|| Mutex::new(Vec::new()));

    
//    let mut locked_vec = mutex_vec.lock().unwrap();
//    locked_vec.push(collision);
//}


//fn process_collision_checks<'a>()-> Vec<bool>{
//    let mutex_vec = COLLISION_COMMANDS.get_or_init(|| Mutex::new(Vec::new())); // Ensure it's initialized
//    let process_data = take_from_mutex_vec(mutex_vec);
//    //let process_data= take_from_mutex_vec(COLLISION_COMMANDS);

//    let mut ret: Vec<bool>= Vec::new();
//    //implement multithreading later
//    for i in process_data{
//        ret.push( i.rect1.collides_with(i.rect2)   );
        
//    }
//    ret 
//}


////impl CollisionBuffer {

   
////    fn new() -> Self {
////        Self { commands: Vec::new() }
////    }

////    fn add_collision_check(&mut self, rect1: usize, rect2: usize) {
////        self.commands.push(CollisionCommand { rect1, rect2 });
////    }

////    fn process(&self, rects: &[Rectangle]) {
////        for command in &self.commands {
////            let r1 = &rects[command.rect1];
////            let r2 = &rects[command.rect2];

////            if r1.collides_with(r2) {
////                println!("Collision detected between {:?} and {:?}", r1, r2);
////            }
////        }
////    }
////}

////let mut collision_buffer = CollisionBuffer::new();



////fn main() {
////    let rects = vec![
////        Rectangle::new(200.0,100.0,100.0,100.0, None),
////        Rectangle::new(100.0,100.0,100.0,100.0, None),
////        Rectangle::new(100.0,100.0,100.0,100.0, None),
////    ];

////    let mut collision_buffer = CollisionBuffer::new();

////    // Instead of checking immediately, we store checks in the buffer
////    for i in 0..rects.len() {
////        for j in (i + 1)..rects.len() {
////            collision_buffer.add_collision_check(i, j);
////        }
////    }

////    // Process all collision checks in one go
////    //collision_buffer.process(&rects);
////}
