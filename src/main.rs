use next_gen::prelude::*;

#[generator(yield(f32))]
pub fn zero_samples() {
    loop {
        yield_!(0f32);
    }
}

#[generator(yield(f32))]
pub fn samples2frame<const S: usize>(mut input: Pin<&mut dyn Generator<Yield = f32, Return = ()>>) {
    let mut buf = Vec::with_capacity(S);
    loop {
        match input.as_mut().resume(()) {
            GeneratorState::Yielded(sample) => {
                buf.push(sample);
            }
            GeneratorState::Returned(r) => {
                return r;
            }
        }
        if buf.len() >= S {
            break;
        }
    }
}

fn main() {
    mk_gen!(let samples = zero_samples());
    // Erroneous syntax error here    V
    mk_gen!(let frames = samples2frame<1024>(samples));
    let frame = frames.as_mut().resume(());
    dbg!(frame);
}
