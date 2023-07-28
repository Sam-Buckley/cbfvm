#[allow(unused_imports)]
use cbvm::{
    builder::bytes::{Byte, ByteStream},
    byte,
    bytecode::{
        data::ByteData,
        ops::ArgType::*,
        ops::Operations::*,
        types::Types::{self, *},
    },
    constant,
    engine::{
        memory::{Heap, Stack},
        Engine,
    },
    op, stream, typed,
};
mod builder;

fn main() {
    let mut _engine = Engine::new_with_size(1024);
    let start = std::time::Instant::now();
    let stream = builder::setup().emitstream(builder::dot());
    println!("{}", stream.stringify());
    println!("Time: {:?}", start.elapsed())
}
