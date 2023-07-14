mod flipper {
    odra::casper::codegen::gen_contract!(something::Flipper, "flipper");
}
mod some_contract {
    odra::casper::codegen::gen_contract!(something::some_contract::SomeContract, "some_contract");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    args.iter().skip(1).for_each(|arg| match arg.as_str() {
        "flipper" => flipper::main(),
        "some_contract" => some_contract::main(),
        _ => println!("Please provide a valid module name!"),
    });
}
