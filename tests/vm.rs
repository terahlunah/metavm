use mana::vm::{emitter::Emitter, function::Functions, value::MetaValue, VM};

#[test]
fn test_factorial() {
    let mut e = Emitter::new();

    let n = e.local_new();
    let total = e.local_new();

    e.local_store(n);
    e.push_int(1).local_store(total);
    e.while_loop(
        |e| {
            e.local_load(n).push_int(1).greater_than();
        },
        |e| {
            e.local_load(n)
                .dup()
                .local_load(total)
                .mul()
                .local_store(total)
                .push_int(1)
                .sub()
                .local_store(n);
        },
    );
    e.local_load(total);

    let factorial = e.finish();
    let mut functions = Functions::new();
    functions.insert("factorial".into(), factorial);

    let mut vm = VM::new(functions);

    vm.push(MetaValue::int(5));
    vm.run("factorial");

    assert_eq!(vm.pop(), Ok(MetaValue::int(120)));
}

#[test]
fn test_closure() {
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

    assert_eq!(vm.pop(), Ok(MetaValue::int(3)));
}

#[test]
fn test_functions() {
    let inc = {
        let mut e = Emitter::new();
        e.push_int(1).add();
        e.finish()
    };
    let add = {
        let mut e = Emitter::new();
        e.add();
        e.finish()
    };
    let add = {
        let mut e = Emitter::new();
        e.add();
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

    let res = vm.pop();
    let expected = Ok(MetaValue::list(vec![6.into(), 7.into()]));

    assert_eq!(res, expected);
}
