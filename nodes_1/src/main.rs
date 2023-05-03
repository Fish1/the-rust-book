fn main() {

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    {
        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });

        println!("leaf = {:?}", leaf);
        println!("branch = {:?}", branch);

        // the upgrade returns none because the weak reference points to nothing
        let leaf_parent = leaf.parent.borrow().upgrade();
        println!("parent value = {:?}", leaf_parent); 

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch cnt weak = {}, strong = {}", Rc::weak_count(&branch), Rc::strong_count(&branch));

        // here the upgrade should return something because we set it equal to Rc::downgrade(&branch)
        // this will increment the strong count of branch by 1 because we are upgrading a weak
        // pointer to a strong pointer. But it goes out of scope after this block...
        {
            let leaf_parent = leaf.parent.borrow().upgrade().expect("we expect a node here...");
            println!("parent value = {:?}", leaf_parent); 
        }

        println!("branch cnt weak = {}, strong = {}", Rc::weak_count(&branch), Rc::strong_count(&branch));
        println!("leaf cnt weak = {}, strong = {}", Rc::weak_count(&leaf), Rc::strong_count(&leaf));
    }


    // the leaf strong pointer was decremented by one when the branch went out of scope
    println!("leaf cnt weak = {}, strong = {}", Rc::weak_count(&leaf), Rc::strong_count(&leaf));
}

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct Node {
    // value of this node
    value: i32,
    // vector of nodes that this node points too
    children: RefCell<Vec<Rc<Node>>>,
    // a parent might be a Weak<T>
    // as a Weak will not increment the counter of a RC
    parent: RefCell<Weak<Node>>
}
