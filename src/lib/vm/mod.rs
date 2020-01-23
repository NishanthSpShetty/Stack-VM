use ::debug;
use lib::stack::VMStack;


const CODE_TYPE_DATA_POSITIVE: u8 = 0;
const CODE_TYPE_DATA_NEGATIVE: u8 = 3; //11

const CODE_TYPE_INSTRUCTION: u8 = 1;

pub struct VM {
    //stack internal state;
    running: bool,
    //program counter
    pc: usize,

    //typ of value read
    type_info: u8,
    //data
    data: i32,

    stack: VMStack,
    //code area
    program_memory: Vec<u32>,
}


impl VM {
    pub fn new(stack_size: usize) -> VM {
        VM {
            program_memory: Vec::new(),
            pc: 0,
            type_info: 0,
            data: 0,
            running: true,
            stack: VMStack::new(stack_size),
        }
    }

    fn is_running(&self) -> bool { self.running }
    fn set_running(&mut self, state: bool) { self.running = state }

    fn current_data(&self) -> i32 { self.data }
    fn current_instruction_type(&self) -> u8 { self.type_info }

    /* load the program to virtual machine memory from
     * the instruction vector
     */
    pub fn load_program(&mut self, instructions: &Vec<u32>) {
        debug!("Loading program...");
        //Insert magic bits to beginning of the program memory, so that we can start pc from 1
        self.program_memory.push(0xBADC0DE);
        for instruction in instructions {
            self.program_memory.push(*instruction);
        }
    }

    /*main thread which executes VM modules
     */
    pub fn run(&mut self) {
        debug!("Memory content : {:?}", self.program_memory);

        debug!("Executing instructions...");
        self.set_running(true);
        while self.is_running() {
            self.fetch();
            self.decode();
            self.exec();
        }
        debug!(" \nExecution completed");
    }


    fn get_type(instruction: u32) -> u8 {
        ((instruction & 0xC0000000_u32) >> 30) as u8//2 msb
    }
    fn get_data(instruction: u32) -> i32 {
        (instruction & 0xffffffff) as i32
    }

    fn fetch(&mut self) {
        //advance the program counter
        if self.pc < self.program_memory.len() {
            self.pc += 1;
        } else {
            panic!("Incomplete code execution, memory boundary reached without reading HALT instruction");
        }
    }
    fn decode(&mut self) {
        let word = self.program_memory[self.pc];
        self.data = VM::get_data(word);
        self.type_info = VM::get_type(word);
    }


    fn exec(&mut self) {
        if self.current_instruction_type() == CODE_TYPE_DATA_POSITIVE || self.current_instruction_type() == CODE_TYPE_DATA_NEGATIVE {
            debug!("Instruction type Data ({} = {} ) ", self.current_instruction_type(), self.current_data());
            self.stack.push(self.data);
        } else {
            //execute instruction
            debug!("Instruction type Operation ({}) , ", self.current_instruction_type());
            self.do_primitive();
        }
    }

    fn do_primitive(&mut self) {
        match self.current_data() & 0xCfffff {
            0 => {
                debug!("[ HALT ] : Stopping VM ");
                self.set_running(false);
                return;
            }
            1 => {
                let top_1 = self.stack.pop();
                let top_2 = self.stack.pop();
                debug!("[ ADD ] : {} {} ", top_1, top_2);
                self.stack.push(top_1 + top_2);
            }
            2 => {
                let top_1 = self.stack.pop();
                let top_2 = self.stack.pop();
                debug!("[ SUB ] : {} {} ", top_1, top_2);
                self.stack.push(top_1 - top_2);
            }
            3 => {
                let top_1 = self.stack.pop();
                let top_2 = self.stack.pop();
                debug!("[ MULT ] : {} {} ", top_1, top_2);
                self.stack.push(top_1 * top_2);
            }
            4 => {
                let top_1 = self.stack.pop();
                let top_2 = self.stack.pop();

                debug!("[ DIV ] : {} {} ", top_1, top_2);
                self.stack.push(top_1 / top_2);
            }
            _ => {
                panic!("[ exec ] : Undefined instruction ");
            }
        }

        debug!(" TOS now : {}", self.stack.peek());
    }
}


#[cfg(test)]
mod test_vm {
    use lib::{vm, vm::VM};

    #[test]
    fn test_get_type() {
        assert_eq!(0, VM::get_type(0x0));
        assert_eq!(vm::CODE_TYPE_INSTRUCTION, VM::get_type(1073741825));
        assert_eq!(vm::CODE_TYPE_DATA_POSITIVE, VM::get_type(22));
        let neg = -100;
        assert_eq!(vm::CODE_TYPE_DATA_NEGATIVE, VM::get_type(neg as u32));
    }

    #[test]
    fn test_get_data() {
        assert_eq!(0, VM::get_data(0));
        assert_eq!(1, VM::get_data(1));
        let num = -1;
        assert_eq!(-1, VM::get_data(num as u32));
    }
}