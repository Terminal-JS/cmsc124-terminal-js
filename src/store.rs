// [FILENAME]: store.rs
// [DESC]:     the shop's products. Kept apart from variable scopes:
//             a product is store state, not a variable.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Product {
    pub price: f64,
    pub stock: f64,
}

#[derive(Default)]
pub struct Store {
    products: HashMap<String, Product>,
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.products.contains_key(name)
    }

    pub fn define(&mut self, name: &str, product: Product) {
        self.products.insert(name.to_string(), product);
    }

    pub fn get(&self, name: &str) -> Option<&Product> {
        self.products.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Product> {
        self.products.get_mut(name)
    }
}