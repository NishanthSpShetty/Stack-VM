extern crate byteorder;

mod lib;


use byteorder::{ByteOrder, LittleEndian};
use lib::stack;

use std::io::prelude::*;
use std::fs::File;


const DEFAULT_RUNTIME_STACK_SIZE :usize = 512;



fn main() {

    //create default runtime stack of defined size
    let mut vm = stack::VM::new(DEFAULT_RUNTIME_STACK_SIZE);

    let mut program: Vec<i32> = Vec::new();

    let argv: Vec<_> = std::env::args().collect();

    if argv.len() != 2 {
        println!("Please enter the source file...");
        println!("USAGE : stack-vm filename");
        return ();
    }

    let mut file = File::open(argv[1].clone()).unwrap();

    let mut buf = [0; 4];
    let len = file.metadata().unwrap().len();
    let mut l = 0;
    while l < len {
        file.read_exact(&mut buf).unwrap();

        let int = LittleEndian::read_i32(&buf[..]);

        program.push(int);
        debug!(" {:?}",program);
        l += 4;
    }


    vm.load_program(&program);
    vm.run();
}
