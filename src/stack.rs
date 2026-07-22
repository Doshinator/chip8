use core::fmt;

const STACK_SIZE: usize = 16;

pub struct Stack {
    stack: [u16; STACK_SIZE],
    sp: u8,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }

    pub fn push(&mut self, value: u16) -> Result<(), StackError> {
        if (self.sp as usize) >= STACK_SIZE {
            return Err(StackError::Overflow { 
                capacity: 16, 
                attempted: self.sp as usize,
            });
        }

        self.stack[self.sp as usize] = value;
        self.sp += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<u16, StackError> {
        if (self.sp as usize) == 0 {
            return Err(StackError::Underflow);
        }

        self.sp -= 1;
        Ok(self.stack[self.sp as usize])
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StackError {
    Overflow { capacity: usize, attempted: usize },
    Underflow, 
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overflow { capacity, attempted } => {
                write!(f, "stack overflow: capacity is {capacity}, attempted to push at index {attempted}")
            },
            Self::Underflow => write!(f, "stack underflow: cannot pop from an empty stack"),
        }
    }
}

/**
 * 
 * STACK TESTS
 * 
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_init() {
        let stack = Stack::new();
        assert_eq!(stack.sp, 0);
        assert_eq!(stack.stack.len(), 16);
        for sp in 0..=15 {
            assert_eq!(0, stack.stack[sp]);
        }
    }

    #[test]
    fn stack_push() {
        let mut stack = Stack::new();

        stack.push(16).unwrap();

        assert_eq!(1, stack.sp);
        assert_eq!(16, stack.stack[0]);
    }

    #[test]
    fn push_full_returns_overflow() {
        let capacity = STACK_SIZE;
        let mut stack = Stack::new();
        for i in 0..16 {
            stack.push(i).unwrap();
        }

        assert_eq!(
            Err(StackError::Overflow {
                capacity,
                attempted: 16,
            }),
            stack.push(99),
        );
    }

    #[test]
    fn stack_pop() {
        let mut stack = Stack::new();

        stack.push(16).unwrap();

        assert_eq!(1, stack.sp);
        assert_eq!(16, stack.pop().unwrap());
        assert_eq!(0, stack.sp);
    }
    #[test]
    fn pop_empty_returns_underflow() {
        let mut stack = Stack::new();
        assert_eq!(
            Err(StackError::Underflow),
            stack.pop(),
        );
    }
}