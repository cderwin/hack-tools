const MEM_SIZE: usize = 32768;

pub enum Register { A, M, D }

pub struct State {
    instructions: Vec<u16>,
    memory: Vec<u16>,
    d: u16,
    a: u16,
    pc: u16
}


impl State {

    pub fn new() -> State {
        State {
            instructions: Vec::with_capacity(MEM_SIZE),
            memory: Vec::with_capacity(MEM_SIZE),
            d: 0 as u16,
            a: 0 as u16,
            pc: 0 as u16
        }
    }

    fn set_addr(&mut self, addr: usize, value: u16) {
        if addr > MEM_SIZE {
            panic!("Attempted to access memory that does not exist");
        }

        if addr > self.memory.len() {
            self.memory.resize(addr, 0 as u16);
        }

        self.memory[addr] = value;
    }

    fn read_addr(&self, addr: usize) -> u16 {
        if addr > MEM_SIZE {
            panic!("Attempted to access memory that does not exist");
        }

        if addr > self.memory.len() {
            return 0 as u16;
        }

        self.memory[addr]
    }

    pub fn write_reg(&mut self, register: Register, value: u16) {
        match register {
            Register::A => self.a = value,
            Register::D => self.d = value,
            Register::M => {
                let addr = self.a as usize;
                self.set_addr(addr, value)
            }
        }
    }

    pub fn read_reg(&self, register: Register) -> u16 {
        match register {
            Register::A => self.a,
            Register::D => self.d,
            Register::M => self.read_addr(self.a as usize)
        }
    }

    pub fn tick(&mut self, jump: bool) {
        if jump {
            self.pc = self.a;
        } else {
            self.pc += 1
        }
    }

    pub fn load_instructions(&mut self, instructions: &mut Vec<u16>) {
        if MEM_SIZE < instructions.len() {
            panic!("Attempted to load more instructions than allowed in ROM");
        }

        self.instructions.clear(); // Remove any instructions in ROM
        self.instructions.append(instructions);
    }

    pub fn fetch_instruction(&self) -> u16 {
        if self.pc as usize > MEM_SIZE {
            panic!("Attempted to access memory that does not exist");
        }

        if self.pc as usize > self.instructions.len() {
            return 0 as u16;
        }

        self.instructions[self.pc as usize]
    }

}
