use mana::vm::{emitter::Emitter, instructions::Inst, value, value::MetaValue, VM};

fn main() {
    let mut emitter = Emitter::new();

    // List.sum
    emitter.emit(Inst::LocalReserve(3));
    emitter.emit(Inst::ListLen);
    emitter.emit(Inst::Dup);
    emitter.emit(Inst::LocalStore(0)); // Len
    emitter.emit(Inst::PushI(0));
    emitter.emit(Inst::LocalStore(1)); // Index
    emitter.emit(Inst::PushI(0));
    emitter.emit(Inst::LocalStore(2)); // Total

    let loop_start = emitter.emit(Inst::Nop);

    emitter.emit(Inst::LocalLoad(1)); // Load Index
    emitter.emit(Inst::PushI(1));
    emitter.emit(Inst::Sub);
    emitter.emit(Inst::LocalStore(1));
    let loop_check = emitter.emit(Inst::LocalLoad(1));
    emitter.emit(Inst::LocalLoad(0));
    emitter.emit(Inst::LessThan);
    emitter.emit(Inst::BranchIf(loop_start));

    // Factorial
    emitter.emit(Inst::LocalReserve(1));
    emitter.emit(Inst::PushI(1));
    emitter.emit(Inst::LocalStore(0)); // total
    let loop_start = emitter.emit(Inst::Dup);
    emitter.emit(Inst::LocalLoad(0));
    emitter.emit(Inst::Mul);
    emitter.emit(Inst::LocalStore(0));
    emitter.emit(Inst::PushI(1));
    emitter.emit(Inst::Sub);
    emitter.emit(Inst::Dup);
    emitter.emit(Inst::PushI(1));
    emitter.emit(Inst::GreaterThan);
    emitter.emit(Inst::BranchIf(loop_start));
    emitter.emit(Inst::Drop);
    emitter.emit(Inst::LocalLoad(0));
    let instructions = emitter.finish();
    let mut vm = VM::new();

    vm.push(MetaValue::int(5));
    vm.execute(instructions);

    println!("Result: {:?}", vm.pop())
}
