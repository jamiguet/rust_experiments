use std::error::Error;
use std::fmt::Debug;
use std::hash::Hash;
use predicates::prelude::*;

#[derive(Debug, Clone)]
pub struct GenSafeInput<'a, T> {
    value: T,
    validator: &'a fn(T) -> bool,
}

impl<'a, T> GenSafeInput<'a, T> 
where T: Clone + PartialEq + Eq + Hash + Debug {
    fn new(value: T, validator: &fn(T) -> bool) -> Result<GenSafeInput<T>, Box<dyn Error>> {
        if validator(value.clone()) {
            Ok (GenSafeInput{ value, validator })
        } else { 
            Err("Incorrect value".into())
        }
    }
    
    fn value(&self) -> &T {
        &self.value
    }
   
    fn validator(&self) -> bool {
        self.validator.clone()(self.value.clone())
    }
}
