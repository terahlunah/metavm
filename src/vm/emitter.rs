use crate::vm::{function::Function, instructions::Inst};

pub struct Emitter {
    instructions: Vec<Inst>,
    locals: usize,
    breaks_if: Vec<usize>,
    breaks_if_not: Vec<usize>,
}

impl Emitter {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
            locals: 0,
            breaks_if: vec![],
            breaks_if_not: vec![],
        }
    }

    pub fn emit(&mut self, inst: Inst) {
        self.instructions.push(inst)
    }

    pub fn previous_idx(&self) -> usize {
        self.instructions.len() - 1
    }

    pub fn current_idx(&mut self) -> usize {
        self.instructions.len()
    }

    pub fn nop(&mut self) -> &mut Self {
        self.emit(Inst::Nop);
        self
    }

    pub fn dup(&mut self) -> &mut Self {
        self.emit(Inst::Dup);
        self
    }

    pub fn drop(&mut self) -> &mut Self {
        self.emit(Inst::Drop);
        self
    }

    pub fn swap(&mut self) -> &mut Self {
        self.emit(Inst::Swap);
        self
    }

    pub fn push_int(&mut self, v: i64) -> &mut Self {
        self.instructions.push(Inst::PushI(v));
        self
    }

    pub fn push_function_ref(&mut self, name: impl Into<String>) -> &mut Self {
        self.instructions.push(Inst::PushFn(name.into()));
        self
    }

    pub fn add(&mut self) -> &mut Self {
        self.emit(Inst::Add);
        self
    }

    pub fn not(&mut self) -> &mut Self {
        self.emit(Inst::Not);
        self
    }

    pub fn list_len(&mut self) -> &mut Self {
        self.instructions.push(Inst::ListLen);
        self
    }

    pub fn list_get(&mut self) -> &mut Self {
        self.instructions.push(Inst::ListGet);
        self
    }

    pub fn list_set(&mut self) -> &mut Self {
        self.instructions.push(Inst::ListSet);
        self
    }

    pub fn call(&mut self) -> &mut Self {
        self.emit(Inst::Call);
        self
    }

    // Todo: reuse freed local idx
    pub fn local_new(&mut self) -> usize {
        let id = self.locals;
        self.locals += 1;
        id
    }

    pub fn local_load(&mut self, local: usize) -> &mut Self {
        self.emit(Inst::LocalLoad(local));
        self
    }

    pub fn local_store(&mut self, local: usize) -> &mut Self {
        self.emit(Inst::LocalStore(local));
        self
    }

    pub fn less_than(&mut self) -> &mut Self {
        self.emit(Inst::LessThan);
        self
    }

    pub fn branch(&mut self, target: usize) -> &mut Self {
        self.emit(Inst::Branch(target));
        self
    }

    pub fn branch_if(&mut self, target: usize) -> &mut Self {
        self.emit(Inst::BranchIf(target));
        self
    }

    pub fn break_if(&mut self) -> &mut Self {
        self.nop();
        self.breaks_if.push(self.previous_idx());
        self
    }

    pub fn break_if_not(&mut self) -> &mut Self {
        self.nop();
        self.breaks_if_not.push(self.previous_idx());
        self
    }

    pub fn patch(&mut self, idx: usize, inst: Inst) {
        if let Some(i) = self.instructions.get_mut(idx) {
            *i = inst;
        }
    }

    pub fn infinite_loop(&mut self, mut block: impl FnMut(&mut Emitter)) -> &mut Self {
        let start = self.current_idx();
        block.call_mut((self,));
        self.branch(start);
        let break_target = self.current_idx();
        for b in self.breaks_if.clone() {
            self.patch(b, Inst::BranchIf(break_target));
        }
        self.breaks_if.clear();
        for b in self.breaks_if_not.clone() {
            self.patch(b, Inst::BranchIfNot(break_target));
        }
        self.breaks_if_not.clear();
        self
    }

    pub fn while_loop(
        &mut self,
        mut cond: impl FnMut(&mut Emitter),
        mut block: impl FnMut(&mut Emitter),
    ) -> &mut Self {
        self.infinite_loop(|e| {
            cond.call_mut((e,));
            e.break_if_not();
            block.call_mut((e,));
        })
    }

    pub fn finish(self) -> Function {
        Function {
            instructions: self.instructions,
            locals: self.locals,
        }
    }
}
