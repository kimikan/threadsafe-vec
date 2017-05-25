Description:
   a thread-safe wrapper vector written in rust!


Usage:

let lck = SortedOrderList::new();
let c = Item{_str:"x".to_owned()};
let c2 = Item{_str:"y".to_owned()};

lck.add(c);
lck.add(c2);

let x:Arc<RefCell<Item>> = lck.find(finder1).unwrap();
println!("Hello, world: {:?}", x.borrow_mut()._str);
