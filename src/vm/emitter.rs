use crate::vm::instructions::Inst;

pub struct Emitter {
    instructions: Vec<Inst>,
}

impl Emitter {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
        }
    }

    pub fn emit(&mut self, inst: Inst) -> usize {
        self.instructions.push(inst);
        self.instructions.len() - 1
    }

    pub fn patch(&mut self, idx: usize, inst: Inst) {
        if let Some(i) = self.instructions.get_mut(idx) {
            *i = inst;
        }
    }

    pub fn nop(&mut self) -> usize {
        self.emit(Inst::Nop)
    }

    pub fn finish(self) -> Vec<Inst> {
        self.instructions
    }
}
