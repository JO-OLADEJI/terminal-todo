use crate::Calldata;
use crate::Memory;
use crate::Stack;
use crate::Storage;
use crate::OPCODES;

pub enum ExecutionTrail {
    Left,
    Right,
}

#[derive(Debug)]
pub struct EVM {
    bytecode: String,
    program_counter: usize,
    command_length: usize,
    pub pc_history: Vec<usize>,

    pub stack: Stack,
    pub memory: Memory,
    pub calldata: Calldata,
    pub storage: Storage,
}

impl EVM {
    pub fn default(bytecode: String) -> EVM {
        EVM {
            bytecode,
            command_length: 2,
            program_counter: 0,
            pc_history: Vec::new(),
            stack: Stack::new(),
            memory: Memory::new(),
            calldata: Calldata::default(),
            storage: Storage::default(),
        }
    }

    pub fn new(
        bytecode: String,
        calldata: Calldata,
        memory: Option<Memory>,
        storage: Option<Storage>,
        stack: Option<Stack>,
    ) -> EVM {
        let defined_memory: Memory;
        let defined_storage: Storage;
        let defined_stack: Stack;

        match memory {
            Some(value) => {
                defined_memory = value;
            }

            None => {
                defined_memory = Memory::new();
            }
        }

        match storage {
            Some(value) => {
                defined_storage = value;
            }

            None => {
                defined_storage = Storage::default();
            }
        }

        match stack {
            Some(value) => {
                defined_stack = value;
            }

            None => {
                defined_stack = Stack::new();
            }
        }

        EVM {
            bytecode,
            calldata,
            command_length: 2,
            program_counter: 0,
            pc_history: Vec::new(),
            memory: defined_memory,
            storage: defined_storage,
            stack: defined_stack,
        }
    }

    pub fn get_bytecode(&self) -> &String {
        &self.bytecode
    }

    pub fn get_program_counter(&self) -> usize {
        self.program_counter
    }

    pub fn get_command_length(&self) -> usize {
        self.command_length
    }

    pub fn update_program_counter(&mut self, command: ExecutionTrail) {
        let current_opcode = self.bytecode
            [self.program_counter..self.program_counter + self.command_length]
            .to_string();

        match command {
            ExecutionTrail::Left => {
                let prev_index = self.pc_history.last();

                if let Some(value) = prev_index {
                    let _ = self.stack.undo();
                    self.program_counter = value.clone();
                    self.pc_history.pop();
                }
            }

            ExecutionTrail::Right => {
                let res = self.execute_opcode(&current_opcode);

                match res {
                    Ok(payload_size) => {
                        let pc_cache = (self.program_counter + self.command_length + payload_size)
                            .min(self.bytecode.len() - self.command_length);

                        if self.program_counter != pc_cache {
                            self.pc_history.push(self.program_counter);
                            self.program_counter = pc_cache;
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

impl EVM {
    pub fn execute_opcode(&mut self, opcode: &str) -> Result<usize, ()> {
        let mut payload_size = 0;
        let i = self.program_counter + self.command_length;

        match opcode {
            "35" => self.CALLDATALOAD(),
            "50" => self.POP(),
            "5f" => self.PUSH0(),
            "63" => {
                payload_size = 8;
                self.PUSH4(&self.bytecode[i..i + payload_size].to_string());
            }
            "73" => {
                payload_size = 40;
                self.PUSH20(&self.bytecode[i..i + payload_size].to_string());
            }
            _ => (),
        }

        Ok(payload_size)
    }
}

impl OPCODES for EVM {
    fn CALLDATALOAD(&mut self) {
        //
    }

    fn POP(&mut self) {
        let _ = self.stack.pop();
    }

    fn PUSH0(&mut self) {
        let _ = self.stack.push(&String::from("0"));
    }

    fn PUSH1(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH2(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH3(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH4(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH5(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH6(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH7(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH8(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH9(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH10(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH11(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH12(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH13(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH14(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH15(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH16(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH17(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH18(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH19(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH20(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH21(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH22(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH23(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH24(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH25(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH26(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH27(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH28(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH29(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH30(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH31(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH32(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }
}
