use std::io::{Read, Write};

use ::state::State;
use super::*;

pub fn interpret<R, W>(instructions: &[Instruction], state: &mut State,
                       input: &mut R, output: &mut W)
    where R: Read, W: Write
{
    let mut pc = 0;

    while pc < instructions.len() {
        match instructions[pc] {
            (OpCode::Left, count) => state.left(count),
            (OpCode::Right, count) => state.right(count),
            (OpCode::Up, count) => state.up(count as u8),
            (OpCode::Down, count) => state.down(count as u8),
            (OpCode::In, count) => {
                for _ in 0 .. count {
                    state.read(input);
                }
            }
            (OpCode::Out, count) => {
                for _ in 0 .. count {
                    state.write(output);
                }
            }
            (OpCode::Begin, count) => {
                if state.load() == 0 {
                    pc = count - 1;
                }
            }
            (OpCode::End, count) => {
                if state.load() != 0 {
                    pc = count;
                }
            }
        }

        pc += 1;
    }
}