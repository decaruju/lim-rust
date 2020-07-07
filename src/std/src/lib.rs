use object::Object;
use std::rc::Rc;

#[no_mangle]
pub fn println(args: Vec<Rc<Object>>) -> Rc<Object> {
    for arg in args {
        println!("{}", arg.to_string());
    }
    Rc::new(Object::None)
}

#[no_mangle]
pub fn test(args: Vec<Rc<Object>>) -> Rc<Object> {
    if let Object::Integer(number) = **args.get(0).unwrap() {
        Rc::new(Object::Integer(fib(number)))
    } else {
        Rc::new(Object::Error(String::from("First argument is not an Integer")))
    }
}

fn fib(n: i64) -> i64 {
    if n == 0 {
        1
    } else if n == 1 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}
