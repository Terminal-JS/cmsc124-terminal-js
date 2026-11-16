// [FILENAME]: environment.rs

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::value::Value;

// type alias for a reference-counted, mutable Environment
pub type EnvRef = Rc<RefCell<Environment>>;

pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<EnvRef>,

}

impl Environment {
    // the global scope
    pub fn new_ref() -> EnvRef {
        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: None,
        }))
    }

    // a block scope sitting inside 'parent'
    pub fn child_of(parent: &EnvRef) -> EnvRef {
        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: Some(Rc::clone(parent)),
        }))
    }


    // defines a product and records it to the hashmap
    pub fn define(&mut self, name: &str, product: Product) {
        self.products.insert(name.to_string(), product);
    }

    // checks if a specific product exists in the hashmap
    pub fn contains_local(&self, name: &str) -> bool {
        self.products.contain_key(name)
    }

    // param:  product name (key)
    // return: product struct (price, stock)
    pub fn get(&self, name: &str) -> Option<&Product> {
        self.products.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Product> {
        todo!()
    }
}