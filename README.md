## Stack-VM - Stack Virtual Machine

Stack Based Virtual Machine  is a type of virtual machgine which uses stacks to perform operation, ex : JVM .

Its a simple implementation of VM which executes the bytecode (32bit so its a wordcode) to understand the JVM.


## Working
Stack Based virtual machine creates runtime stack. VM only performs two operations directly on Java Stacks: it pushes and pops data.
While reading wordcode if the instruction is encountered it will pop 2 operand from the top of the stack and perform operation on it, push the result back to stack. If encountered word is data it will push to the stack. 


## Stack-VM Instruction set

VM executes postfix expression compiled into 32 bit long VM instructions. 

First 2 bits distinguishes between data and instruction.

|bits| Type |
|----|-----------------|
| 00 | Data (Positive number)|
| 01 | Operation Instruction (OI)|
| 10 | Data (negative number)|
| 11 | undefined|

## VM operation instruction (OI)
----------------------------------------
    0x40000001		ADD           
    0x40000010		SUB           
    0x40000011	   	MUL           
    0x40000100	   	DIV           
    0x40000000		HALT
    
All program must end with `HALT` instruction.

VM Byte code can be generated from the `stack-langc` [compiler](https://github.com/NishanthSpShetty/stack-langc).


## Run
This repo contains file `a.out` with VM instruction to add two numbers, you can dump hex content to view the data.
Byteorder is in little endian, I'll explain the word stored in the file.

Hex word | VM Value |
|----|----|
|0x00000020| Data (32 in decimal)|
|0x00000016| Data (22 in decimal)|
|0x40000001| OI (ADD)|
|0x40000000| OI (HALT)|

Run the above program.

    cargo run a.out
    
In debug mode you can see the result 54 at the top of the stack.

