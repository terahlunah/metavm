use mana::vm::{
    emitter::Emitter,
    env::Env,
    function::Functions,
    instructions::Inst,
    value,
    value::{FunctionRef, MetaValue},
    VM,
};

fn main() {
    let add_n_closure = {
        let mut e = Emitter::with_env(1);
        e.local_load(0).add();
        e.finish()
    };
    let add_n = {
        let mut e = Emitter::new();
        e.push_list()
            .swap()
            .list_push()
            .push_function_ref("add_n_closure")
            .swap()
            .bind();
        e.finish()
    };
    let main_fn = {
        let mut e = Emitter::new();
        e.push_int(1)
            .push_int(2)
            .push_function_ref("add_n")
            .call()
            .call();

        e.finish()
    };

    let mut functions = Functions::new();
    functions.insert("add_n_closure".into(), add_n_closure);
    functions.insert("add_n".into(), add_n);
    functions.insert("main".into(), main_fn);

    let mut vm = VM::new(functions);

    vm.run("main");

    println!("Result: {:?}", vm.pop())
}
