// this version is not thread-safe implementation
//for Item access
//the if change Arc<RefCell<Item>> to Arc<RwLock<Item>>, it's better
//or just remove RefCell.   let the item shares the same RwLock with List
// this example just show these differences

use std::sync::RwLock;
use std::sync::Arc;

pub struct Item {
    pub _str: String,
}

struct SortedOrderList {
    _vec: RwLock<Vec<Arc<RwLock<Item>>>>,
}

impl SortedOrderList {

    fn new() -> SortedOrderList {
        let vec: Vec<Arc<RwLock<Item>>> = vec![];
        SortedOrderList{
            _vec : RwLock::new(vec) 
        }
    }

    fn add(&self, c: Item) {
        match self._vec.write() {
            Ok(mut e) => {
                e.push(Arc::new(RwLock::new(c)));
            }
            Err(_) => {}
        }
    }

    pub fn find(&self,
                f: fn(Arc<RwLock<Item>>) -> bool)
                -> Option<Arc<RwLock<Item>>> {
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

fn finder1(c:Arc<RwLock<Item>>)->bool{
    c.read().unwrap()._str == "x" 
}

fn main() {
    let lck = SortedOrderList::new();
    let c = Item{_str:"x".to_owned()};
    let c2 = Item{_str:"y".to_owned()};

    lck.add(c);
    lck.add(c2);

    let x:Arc<RwLock<Item>> = lck.find(finder1).unwrap();

    println!("Hello, world: {:?}", x.read().unwrap()._str);
}
