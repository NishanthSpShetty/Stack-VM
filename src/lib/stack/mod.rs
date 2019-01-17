/* Stack Virtual Machine
 * This module defines stack and various
 * Operations on it.
 */
macro_rules! debug {
    ($($p:tt)*) => (if cfg!(debug_assertions) { println!($($p)*) } else { () })
}

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

	/* Initialize virtual machine
	 *Setup stack pointer , PC
	 */
	pub fn new()->VM{
		let mem_allot = vec![0;64]; //create a stack of size 512 i32
		VM{memory:mem_allot, pc:31,sp:0,typ:0,dat:0,running:true }	
	}

	
	/*load the program to virtual machine memory from
	 * the instruction vector
	 */
	pub fn load_program(&mut self,prog:&Vec<i32>){
		debug!("Loading program...");
		let mut loc = 1;
		for i in prog{
			self.memory.insert(self.pc+loc,*i);
			loc+=1;
		}
//		debug!("Memory content : {:?}",self.memory);
	
	}

	/*main thread which executes VM modules
	 */
	pub fn run(&mut self) {
		debug!("Executing instructions...");
		while self.running {
			self.fetch();
			self.decode();
			self.exec();
		}
		debug!(" \nExecution completed");
	}


	fn get_type(&self, instruction:i32)->i32{
		(instruction & 0xC0000000_u32 as i32)>>30//2 msb	
		
	}
	fn get_data(&self, instruction:i32)->i32{
		instruction & 0x3fffffff
		}

	fn fetch(&mut self){
		if self.pc < 63 {
			self.pc+=1;	
		}else{
			panic!("Incomplete code execution, memory boudary reached without reading HALT instruction");
		}
	}
	fn decode(&mut self){
		let i = self.pc;
		self.dat = self.get_data(self.memory[i]);
		self.typ = self.get_type(self.memory[i]);
//		debug!("{:?}",self.memory);
	}


		
	fn exec(&mut self){
		if self.typ == 0 || self.typ == 2 {
			/* the data is a operand
			 * push on to the stack 
			 */
			 self.memory[self.sp]=self.dat;
			 self.sp+=1;
		 }else{
		 	self.do_primitive();
		 }
	}

	fn do_primitive(&mut self){
		
		
		match self.dat {
			0 =>{
				debug!("[ HAL:exec ]");
				self.running = false;
				return;
			},
			1 =>{
				let top_1 = self.memory[self.sp-1];
				let top_2 = self.memory[self.sp-2];
				debug!("[ ADD ] : {} {} ",top_1,top_2);
				self.memory[self.sp-2] = top_1 + top_2;
				self.sp-=1;
			},
			2 =>{
				let top_1 = self.memory[self.sp-1];
				let top_2 = self.memory[self.sp-2];
				debug!("[ SUB ] : {} {} ",top_1,top_2);
				self.memory[self.sp-2] = top_1 - top_2;
				self.sp-=1;
			
			},
			3 => {
				let top_1 = self.memory[self.sp-1];
				let top_2 = self.memory[self.sp-2];
				debug!("[ MULT ] : {} {} ",top_1,top_2);
				self.memory[self.sp-2] =top_1 * top_2;
				self.sp-=1;
			
			},
			4 => {
				let top_1 = self.memory[self.sp-1];
				let top_2 = self.memory[self.sp-2];
				
				debug!("[ DIV ] : {} {} ",top_1,top_2);
				self.memory[self.sp-2]  = top_1 / top_2;
				self.sp-=1;
			},
			_ => {
				panic!("[ exec ] : Undefined instruction ");
			},
		
		}
		
		debug!(" TOS now : {}",self.memory[self.sp-1]);
		
			
	}
}
