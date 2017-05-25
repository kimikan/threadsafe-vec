
use std::sync::RwLock;
use std::sync::Arc;
use std::cell::RefCell;

pub struct Item {
    pub _str: String,
}

struct SortedOrderList {
    _vec: RwLock<Vec<Arc<RefCell<Item>>>>,
}

impl SortedOrderList {

    fn new() -> SortedOrderList {
        let vec: Vec<Arc<RefCell<Item>>> = vec![];
        SortedOrderList{_vec : RwLock::new(vec) }
    }

    fn add(&self, c: Item) {
        match self._vec.write() {
            Ok(mut e) => {
                e.push(Arc::new(RefCell::new(c)));
            }
            Err(_) => {}
        }
    }

    pub fn find(&self,
                f: fn(Arc<RefCell<Item>>) -> bool)
                -> Option<Arc<RefCell<Item>>> {
        match self._vec.write() {
            Ok(ref mut e) => {
                let mut index: i32 = -1;
                for elem in e.iter() {
                    index += 1;
                    if f(elem.clone()) {
                        break;
                    }
                }

                if index >= 0 {
                    Some(e[index as usize].clone())
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}

fn finder1(c:Arc<RefCell<Item>>)->bool{
    c.borrow_mut()._str == "x" 
}

fn main() {
    let lck = SortedOrderList::new();
    let c = Item{_str:"x".to_owned()};
    let c2 = Item{_str:"y".to_owned()};

    lck.add(c);
    lck.add(c2);

    let x:Arc<RefCell<Item>> = lck.find(finder1).unwrap();

    println!("Hello, world: {:?}", x.borrow_mut()._str);
}