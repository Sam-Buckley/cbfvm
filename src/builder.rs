use cbvm::{
    builder::bytes::{Byte, ByteStream},
    byte,
    bytecode::types::Types,
    op, stream, typed,
};

pub fn setup() -> ByteStream {
    ByteStream::new()
        .emit(op!(ALLOC))
        .emitstream(stream![(TypeReg, 0x1), (TypeU8, 128)])
        //Above is register 1 for the strip, below is the pointer (reg 0)
        .emit(op!(MOV))
        .emitstream(stream! {
            (TypeU8, 0x0), (TypeU8, 0x0)
        })
}

pub fn dot() -> ByteStream {
    ByteStream::new()
        .emit(op!(ALLOC))
        .emitstream(stream!((TypeU8, 0x2), (TypeU8, 0x1)))
        .emit(op!(READ))
        .emitstream(stream!(
            (TypeU8, 0x3),
            (TypeReg, 0x0),
            (TypeU8, 0x1) //dest, target, size (1 byte)
        ))
        .emit(op!(STORE))
        .emitstream(stream! {
            (TypeReg, 0x2), (TypeReg, 0x3) //destination (mem), source (reg)
        })
        .emit(op!(WRITE))
        .emitstream(stream!((TypeU8, 0x3), (TypeU8, 1))) //write is weird and needs to be updated, reads reg for you.
}
