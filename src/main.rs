mod lib;
use lib::stack;

use std::io;
use std::io::prelude::*;
use std::fs::File;
fn main() {
	let mut vm = stack::VM::new();
	let mut program:Vec<i32> = Vec::new();
	let argv:Vec<_> = std::env::args().collect(); 
  	if argv.len() != 2 {
		println!("Please enter the source file...");
		println!("USAGE : stack-vm filename");
		return();
	}
	
	//println!(" {:?}",argv);
	
	let mut file = File::open(argv[1].clone()).unwrap();


	let mut buf = [0;4];
	let len = file.metadata().unwrap().len();
	let mut l=0;
	while l <len {
		file.read_exact(&mut buf).unwrap();
		let int = (buf.iter().rev().fold(0,|acc,&b|(acc<<1)+b as u32)) as i32;
		println!(" {:b}",int );
		program.push(int);
//		println!(" {:?}",program);
		l+=4;
	}

	vm.load_program(&program);
	vm.run();
   }
