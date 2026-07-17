const STACK_SIZE: usize = 16;
pub struct Stack {
    stack: [u16; STACK_SIZE],
    sp: u8,
}

impl Stack {
    fn top(&self) -> u16 {
        self.stack[self.sp as usize]
    }
}