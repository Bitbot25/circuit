use std::collections::HashMap;

use super::object::CircuitObject;

pub struct Environment {
    values: HashMap<String, CircuitObject>
}

impl Environment {
    pub fn new() -> Environment {
        Environment { values: HashMap::new() }
    }

    pub fn with_native() -> Environment {
        let env = Self::new();

        env
    }

    pub fn define(&mut self, name: String, obj: CircuitObject) {
        self.values.insert(name, obj);
    }

    pub fn get(&self, name: &String) -> Option<&CircuitObject> {
        self.values.get(name)
    }
}










