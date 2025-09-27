

pub struct pHashMap<K,V>{
	data: Vec<(K,V)>,
	allocated_size: usize,
}

impl <K,V> pHashMap<K,V>{
	pub fn new()-> Self{
		pHashMap{
			data: Vec::new(),
			allocated_size: 1000
		}
	}
	pub fn get(key: K){

	}
	pub fn push(key: K, value: V){
		
	}
	fn hash(&self)->usize{
		5
	}
}