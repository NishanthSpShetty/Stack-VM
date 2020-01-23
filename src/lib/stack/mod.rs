pub struct VMStack {
    internal_stack: Vec<i32>,
    capacity: usize,
    top: usize, //also size
}

impl VMStack {
    pub fn new(stack_size: usize) -> VMStack {
        VMStack { capacity: stack_size, top: 0, internal_stack: Vec::with_capacity(stack_size) }
    }

    pub fn push(&mut self, data: i32) {
        if self.top == self.capacity {
            panic!("Stack overflow Capacity {} , Size {} ", self.capacity, self.top)
        }
        self.internal_stack.push(data);
        self.top += 1;
    }

    pub fn pop(&mut self) -> i32 {
        if self.top == 0 {
            panic!("Stack underflow")
        }
        self.top -= 1;
        self.internal_stack.pop().unwrap()
    }

    pub fn peek(&self) -> i32 { *self.internal_stack.last().unwrap() }
}


#[cfg(test)]
mod test_vmstack {
    use lib::stack::VMStack;

    #[test]
    fn test_vm_stack_push_pop() {
        let mut stack = VMStack::new(1);
        stack.push(10);

        assert_eq!(10, stack.peek());
        assert_eq!(10, stack.pop());
    }

    #[test]
    #[should_panic]
    fn test_peek_when_empty_expect_panic() {
        let stack = VMStack::new(1);
        assert_eq!(10, stack.peek());
    }

    #[test]
    #[should_panic]
    fn test_push_when_full_expect_overflow() {
        let mut stack = VMStack::new(1);
        stack.push(1);
        stack.push(2);
    }

    #[test]
    #[should_panic]
    fn test_pop_when_empty_epect_underflow(){
        let mut stack = VMStack::new(1);
        assert_eq!(10, stack.pop());
    }
}