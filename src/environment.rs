// [FILENAME]: environment.rs

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// a store item's two fields
pub struct Product {
    pub price: f64,
    pub stock: f64,
}


// a products symbol table
pub struct Environment {
    products: HashMap<String, Product>,
    enclosing: Option<Rc<RefCell<Environment>>>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            products: Hashmap::new(),
            enclosing: None,
        }
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