/* Stack Virtual Machine
 * This module defines stack and various
 * Operations on it.
 */

pub struct VM{
	//stack registers;
	pc:usize,//program counter
	sp:usize,//stack pointer
	typ:i32,//typ of value read
	dat:i32,//data
	running:bool,
	memory:Vec<i32>
}

impl VM{
	pub fn say_hello(&self){println!("Works fine...");}

	pub fn new()->VM{
		let mem_allot = vec![0,512]; //create a stack of size 512 i32
		VM{memory:mem_allot, pc:100,sp:0,typ:0,dat:0,running:true }	
	}

	pub fn run(&mut self) {
		self.pc-=1; //initial fetch will load program counter to appropriate address
		println!("Executing instructions...");
		while self.running {
			self.fetch();
			self.decode();
			self.exec();
		}
		println!(" \nExecution completed");
	}

	pub fn load_program(&mut self,prog:&Vec<i32>){
		println!("Loading program...");
		for i in prog{
			self.memory.push(*i);
		}
		println!("Memory content : {:?}",self.memory);
	
	}


	fn get_type(&self, instruction:i32)->i32{
		(instruction& 0xc0000000)>>30 //2 msb	
	}
	fn get_data(&self, instruction:i32)->i32{
		instruction & 0x3fffffff
	}

	fn fetch(&mut self){
		self.pc+=1;	
	}
	fn decode(&mut self){
		self.dat = self.get_data(self.memory[self.pc]);
		self.typ = self.get_type(self.memory[self.pc]);
	}
		
	fn exec(&mut self){
		println!(" TOS now : {}",self.memory[self.sp-1]);
		if self.typ == 0 || self.typ == 2 {
		/* the data is a operand
		 * push on to the stack 
		 */
		 self.memory.insert(self.sp,self.dat);
		 self.sp+=1;
		 }else{
		 	self.do_primitive();
		 }
	}
	fn do_primitive(&mut self){
		println!("Doing primitive ops");
		self.memory.remove(self.sp-1);
		self.memory.remove(self.sp-1);
			
	}
}
