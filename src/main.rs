mod lib;
use lib::stack;

fn main() {
    println!("Hello, world!");

    let vm = stack::VM::new();
   vm.say_hello(); 
   }
