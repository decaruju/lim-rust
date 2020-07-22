use std::collections::HashMap;
use std::rc::Rc;
use parser::node::Node;
extern crate libloading;

#[derive(Debug, Clone)]
pub struct Object {
    pub fields: HashMap<String, Rc<Object>>,
    pub native: Option<Native>
}

#[derive(Debug, Clone)]
pub enum Native {
    Integer(i64),
    Float(f64),
    NativeCode(String, String),
    Function(Vec<Node>, Vec<Node>),
}

impl Object {
    pub fn get(&self, name: &str) -> Rc<Object> {
        if let Some(object) = self.fields.get(name) {
            Rc::clone(object)
        } else {
            panic!("Field {:?} was not found on object {:?}", name, self)
        }
    }

    pub fn is_native(&self) -> bool {
        self.native.is_some()
    }

    pub fn to_string(&self) -> String {
        if let Some(native) = &self.native {
            match native {
                Native::Integer(integer) => format!("{}", integer),
                Native::Float(float) => format!("{}", float),
                Native::NativeCode(_, _) => format!("[native code]"),
                Native::Function(_, _) => format!("function"),
            }
        } else {
            format!("{:?}", self)
        }
    }
    // pub fn NativeClass() -> Rc<Object> {
    //     let mut native_class = Object{
    //         fields: HashMap::new(),
    //     }

    //     native_class.insert(String::from("$class"), Object::ClassClass());
    // }

    // pub fn Native(library: String, symbol: String) -> Rc<Object> {
    //     let mut native = Object{
    //         fields: HashMap::new(),
    //     };
    //     native.fields.insert("library", library);
    //     native.fields.insert("symbol", symbol);

    //     Rc::new(native)
    // }

    // pub fn None
}


impl Default for Object {
    fn default() -> Self {
        Object{
            fields: HashMap::new(),
            native: None,
        }
    }
}
