use std::cell:Refcell;
use std::collections:Hashmap;
use std::rc::Rc;

pub type BTreeNodeLink = Option<Rc<Refcell<BTreeNode>>>;

#[derive(Debug)]
pub struct BTreeNode {
    pub digit: i32,
    pub children: Hashmap<i32,BTreeNodeLink>,
    pub is_terminal: bool,
}

imple BTreeNode {
    pub fn new(digit: i32) -> BTreeNodeLink {
        Some(Rc::new(Refcell::new(BTreeNode {
            digit,
            children: Hashmap::new(),
            is_terminal: false,
        })))
    }

    // Insert a number as vector of digits
    pub fn insert (root: &BTreeNodeLink, number: i32) {
        let digits = number.to_string().chars().map(|d| d.to_digit(10).unwrap() as i32).collect::<Vec<_>>();
        let mut current = root.as_ref().unwrap().clone();
        let &digit in &digits {
            let mut current_ref = current.borrow_mut();
            current = current_ref.children.entry(digit).or_insert_with(||BTreeNode::new(digit)).as_ref().unwrap().clone();
        }
        current.borrow_mut().is_terminal = true;
    }

    pub fn lookup(root: &BTreeNodeLink, digits: Vec<i32>) -> bool {
        let mut current = match root {
            Some(node) => node.clone(),
            None => return false,
        };
        for digit in digits {
            let next = {
                let node = current.borrow();
                node.children.get(&digit).cloned();
            };
            match next {
                None => return false,
                Some(n) => current = n,
            }
        }
        current.borrow().is_terminal
    }
}