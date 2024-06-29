#![allow(unused)]

use inkwell::context::Context;

fn main() {
    let context = Context::create();
    let module = context.create_module("module");
    let builder = context.create_builder();

    context.f64_type().const_float(0.0);
    // context.append_basic_block(function, "then");
}
