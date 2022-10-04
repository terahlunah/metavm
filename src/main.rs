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
    let inc = {
        let mut e = Emitter::new();
        e.push_int(1).add();
        e.finish()
    };
    let list_inc = {
        let mut e = Emitter::new();
        e.push_function_ref("inc")
            .push_function_ref("List.map")
            .call();

        e.finish()
    };

    let list_map = {
        let mut e = Emitter::new();

        let f = e.local_new();
        let l = e.local_new();
        let length = e.local_new();
        let i = e.local_new();

        // Init
        e.local_store(f)
            .dup()
            .local_store(l)
            .list_len()
            .local_store(length)
            .push_int(0)
            .local_store(i);

        // Loop
        e.while_loop(
            |e| {
                e.local_load(i).local_load(length).less_than();
            },
            |e| {
                e.local_load(l).local_load(i).list_get();
                e.local_load(f).call();
                e.local_load(l)
                    .swap()
                    .local_load(i)
                    .swap()
                    .list_set()
                    .local_store(l);
                e.local_load(i).push_int(1).add().local_store(i);
            },
        )
        .local_load(l);

        e.finish()
    };

    let mut functions = Functions::new();
    functions.insert("inc".into(), inc);
    functions.insert("List.map".into(), list_map);
    functions.insert("List.inc".into(), list_inc);

    let mut vm = VM::new(functions);

    vm.push(MetaValue::list(vec![5.into(), 6.into()]));
    vm.run("List.inc");

    println!("Result: {:?}", vm.pop())
}
