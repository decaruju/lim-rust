use object::Object;
use std::rc::Rc;

#[no_mangle]
pub fn print(args: Vec<Rc<Object>>) -> Rc<Object> {
    for arg in args {
        println!("{}", arg.to_string());
    }
    Rc::new(Object::default())
}

// #[no_mangle]
// pub fn test(args: Vec<Rc<Object>>) -> Rc<Object> {
//     if let Object::Integer(number) = **args.get(0).unwrap() {
//         Rc::new(Object::Integer(fib(number)))
//     } else {
//         Rc::new(Object::Error(String::from("First argument is not an Integer")))
//     }
// }

// #[no_mangle]
// pub fn native(args: Vec<Rc<Object>>) -> Rc<Object> {
//     let rtn = if let Object::String(lib) = &**args.get(0).unwrap() {
//         if let Object::String(symbol) = &**args.get(1).unwrap() {
//             Object::Native(
//                 lib.to_owned(),
//                 symbol.to_owned()
//             )
//         } else {
//             Object::Error(String::from("Wrong arguments"))
//         }
//     } else {
//         Object::Error(String::from("Wrong arguments"))
//     };
//     Rc::new(rtn)
// }

// fn fib(n: i64) -> i64 {
//     if n == 0 {
//         1
//     } else if n == 1 {
//         1
//     } else {
//         fib(n-1) + fib(n-2)
//     }
// }
