# Stack Virtual Machine

A simple VM written in Rust. A stack based VM illustrating execution of stack based VM's with basic arithmetic operations

VM executes postfix expression compiled into 32 bit long VM instruction format

## 32 bit instructions
where first 2 bit defines type as follows
```
 00 => +ve number
 01 => primitive operation
 10 => -ve operation
 11 => undefined
```

## Example
------------------------------------------------------------
`0xD			+13`
`0x8000000D       	-13`

## VM instruction
-----------------------------------------------------------
 `0x40000001		ADD`           
 `0x40000010		SUB`           
 `0x40000011	   	MUL`           
 `0x40000100	   	DIV`           
 `0x40000000		HALT`


-------------------------------------------------------------
use [stack-langc](https://github.com/NishanthSpShetty/stack-langc) generate the instruction. 

