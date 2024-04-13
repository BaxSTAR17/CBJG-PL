pub struct Environment {
    pub i32constants: std::collections::HashMap<String, i32>,
    pub i32variables: std::collections::HashMap<String, i32>,
    pub global_funcs: Vec<String>
}

impl Environment {
    pub fn create_env() -> Environment {
        let mut global_funcs: Vec<String> = Vec::new();
        global_funcs.push("helloCarlos".to_string());
        Environment { i32constants: std::collections::HashMap::new(), i32variables: std::collections::HashMap::new(), global_funcs }
    }

    pub fn declare_variable(&mut self, name: String, value: crate::interpreter::RuntimeResult) -> i32 {
        if self.i32variables.contains_key(&name) || self.i32constants.contains_key(&name) { panic!("Variable `{}` already declared", name) }
        match value {
            crate::interpreter::RuntimeResult::NumericValue(x) => self.i32variables.insert(name, x.value).unwrap_or_default(),
            _ => panic!("Can't declare variable '{}' with the value", name)
        }
    }

    pub fn call_variable(&mut self, name: String) -> crate::interpreter::RuntimeResult {
        if self.i32variables.contains_key(&name) { crate::interpreter::RuntimeResult::NumericValue(crate::interpreter::NumericValue { value: self.i32variables.get(&name).unwrap().clone() }) }
        else if self.i32constants.contains_key(&name) { crate::interpreter::RuntimeResult::NumericValue(crate::interpreter::NumericValue { value: self.i32constants.get(&name).unwrap().clone() }) }
        else { panic!("Variable '{}' doesn't exist", name) }
    }

    pub fn set_variable(&mut self, name: String, value: crate::interpreter::RuntimeResult) -> i32 {
        if self.i32variables.contains_key(&name) || self.i32constants.contains_key(&name) {
            match value {
                crate::interpreter::RuntimeResult::NumericValue(x) => self.i32variables.insert(name, x.value).unwrap(),
                _ => panic!("Can't assign variable '{}' with the value", name)
            }
        }
        else { panic!("Variable '{}' does not exist", name) }
    }

    pub fn call_function(&mut self, fname: String, values: Vec<crate::interpreter::RuntimeResult>) -> i32 {
        let functionname = fname.as_str();
        if self.global_funcs.contains(&fname) {
            match functionname {
                "helloCarlos" => print_output(values),
                _ => panic!("global function '{}' not found", fname)
            }
        }
        else { panic!("Can't call function '{}'", fname)}
    }
}

fn print_output(values: Vec<crate::interpreter::RuntimeResult>) -> i32 {
    for result in values { 
        match result {
            crate::interpreter::RuntimeResult::NumericValue(x) => println!("{}", x.value),
            crate::interpreter::RuntimeResult::StringValue(x) => println!("{}", x.value),
            _ => panic!("Cannot print out value")
        }
    }
    1
}

