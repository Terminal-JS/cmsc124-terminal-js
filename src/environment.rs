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

    // creates a binding . returns false (and changes nothing) 
    // if the name already exists in this scope
    pub fn define(&mut self, name: &str, value: Value) -> bool {
        if self.values.contains_key(name) {
            false
        } else {
            self.values.insert(name.to_string(), value);
            true
        }
    }

    // reads a binding, innermost first
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.values.get(name) {
            return Some(v.clone());
        }
        match &self.enclosing {
            Some(outer) => outer.borrow().get(name),
            None => None,
        }
    }

    // updates a binding, innermost first. returns false if the name is not found
    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        if let Some(slot) = self.values.get_mut(name) {
            *slot = value;
            true
        } else {
            match &self.enclosing {
                Some(outer) => outer.borrow_mut().assign(name, value),
                None => false,
            }
        }
    }
}