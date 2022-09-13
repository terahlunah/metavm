use mana::vm::emitter::Emitter;
use mana::vm::instructions::Inst;
use mana::vm::value::MetaValue;
use mana::vm::{value, VM};

fn main() {
    let mut emitter = Emitter::new();

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

    let fact_instructions = emitter.finish();

    let mut vm = VM::new();

    vm.push(MetaValue::int(5));
    vm.execute(fact_instructions);

    println!("Result: {:?}", vm.pop())
}
