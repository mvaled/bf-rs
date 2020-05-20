//! Contains the Interpretable trait, which provides a common interface for running a Brainfuck
//! program.

use std::io::{stdin, stdout, Cursor, Read, Write};

use common::BfResult;
use state::State;

pub use bytecode::BytecodeCompilable;
#[cfg(feature = "jit")]
pub use jit::JitCompilable;
#[cfg(feature = "llvm")]
pub use llvm::LlvmCompilable;
pub use peephole::PeepholeCompilable;
pub use rle::RleCompilable;

/// Program forms that can be interpreted.
pub trait Interpretable {
    /// Interprets a program against the given state.
    fn interpret_state<R: Read, W: Write>(&self, state: State, input: R, output: W)
        -> BfResult<()>;

    /// Interprets a program. If the given `size` is `None`, the default memory size.
    fn interpret<R: Read, W: Write>(
        &self,
        size: Option<usize>,
        input: R,
        output: W,
    ) -> BfResult<()> {
        let state = size.map(State::with_capacity).unwrap_or_else(State::new);
        self.interpret_state(state, input, output)
    }

    /// Interprets a program using stdin and stdout for input and output.
    fn interpret_stdin(&self, size: Option<usize>) -> BfResult<()> {
        self.interpret(size, stdin(), stdout())
    }

    /// Interprets a program from memory, returning a vector of its output.
    fn interpret_memory(&self, size: Option<usize>, input: &[u8]) -> BfResult<Vec<u8>> {
        let input = Cursor::new(input);
        let mut output = Cursor::new(Vec::new());

        self.interpret(size, input, &mut output)?;
        Ok(output.into_inner())
    }
}

/// For converting smaller numeric types into `usize`.
pub trait IntoUsize {
    fn into_usize(self) -> usize;
}

impl IntoUsize for usize {
    fn into_usize(self) -> usize {
        self
    }
}

impl IntoUsize for u64 {
    fn into_usize(self) -> usize {
        self as usize
    }
}

impl IntoUsize for u32 {
    fn into_usize(self) -> usize {
        self as usize
    }
}

impl IntoUsize for u16 {
    fn into_usize(self) -> usize {
        self as usize
    }
}

impl IntoUsize for u8 {
    fn into_usize(self) -> usize {
        self as usize
    }
}
