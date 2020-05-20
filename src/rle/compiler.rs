use super::*;
use ast;

/// Program forms that can be compiled to the RLE AST.
pub trait RleCompilable {
    /// Convert the given program to unoptimized AST to prepare for run-length encoding.
    fn with_ast<F, R>(&self, k: F) -> R
    where
        F: FnOnce(&ast::Program) -> R;

    /// Run-length encode the given program.
    fn rle_compile(&self) -> Box<Program> {
        self.with_ast(compile)
    }
}

/// Compiles an unoptimized [`ast`](../ast/index.html) program to a run-length encoded program.
pub fn compile(program: &ast::Program) -> Box<Program> {
    let mut compiler = Compiler::new();
    compiler.compile(program);
    compiler.into_program()
}

/// Represents the state of an RLE compiler from `ast::Instruction` to `Instruction`.
pub struct Compiler {
    instructions: Vec<Statement>,
    last_command: Command,
    last_repeat: Count,
}

impl Compiler {
    /// Creates a new RLE compiler.
    pub fn new() -> Self {
        Compiler {
            instructions: Vec::new(),
            last_command: Command::Right,
            last_repeat: 0,
        }
    }

    /// Compiles the given sequence of instructions.
    pub fn compile(&mut self, program: &[ast::Statement]) {
        for instruction in program {
            match *instruction {
                ast::Statement::Cmd(op_code) => self.issue_op(op_code),
                ast::Statement::Loop(ref body) => self.issue_loop(compile(body)),
            }
        }
    }

    /// Extracts the compiled program.
    pub fn into_program(mut self) -> Box<Program> {
        self.push_op();
        self.instructions.into_boxed_slice()
    }

    fn push_op(&mut self) {
        if self.last_repeat > 0 {
            self.instructions
                .push(Statement::Cmd(self.last_command, self.last_repeat));
            self.last_command = Command::Right;
            self.last_repeat = 0;
        }
    }

    fn issue_op(&mut self, cmd: Command) {
        if cmd == self.last_command {
            if let Some(last_repeat) = self.last_repeat.checked_add(1) {
                self.last_repeat = last_repeat;
            } else {
                self.push_op();
                self.last_repeat = 1;
            }
        } else {
            self.push_op();
            self.last_command = cmd;
            self.last_repeat = 1;
        }
    }

    fn issue_loop(&mut self, body: Box<Program>) {
        self.push_op();
        self.instructions.push(Statement::Loop(body));
    }
}

#[cfg(test)]
mod tests {
    use super::Command::*;
    use super::Statement as Obj;
    use super::*;
    use ast::Statement as Src;

    #[test]
    fn right_compiles() {
        assert_compile(&[Src::Cmd(Right)], &[Obj::Cmd(Right, 1)]);
    }

    #[test]
    fn three_rights_compile() {
        assert_compile(
            &[Src::Cmd(Right), Src::Cmd(Right), Src::Cmd(Right)],
            &[Obj::Cmd(Right, 3)],
        );
    }

    #[test]
    fn two_rights_two_ups_compile() {
        assert_compile(
            &[Src::Cmd(Right), Src::Cmd(Right), Src::Cmd(Up), Src::Cmd(Up)],
            &[Obj::Cmd(Right, 2), Obj::Cmd(Up, 2)],
        );
    }

    #[test]
    fn loop_compiles() {
        assert_compile(
            &[
                Src::Cmd(In),
                src_mk_loop(vec![Src::Cmd(Right)]),
                Src::Cmd(In),
            ],
            &[
                Obj::Cmd(In, 1),
                mk_loop(vec![Obj::Cmd(Right, 1)]),
                Obj::Cmd(In, 1),
            ],
        );
    }

    fn assert_compile(src: &[ast::Statement], expected: &[Statement]) {
        let actual = compile(src);
        assert_eq!(&*actual, expected);
    }

    fn src_mk_loop(body: Vec<Src>) -> Src {
        Src::Loop(body.into_boxed_slice())
    }

    fn mk_loop(body: Vec<Statement>) -> Statement {
        Obj::Loop(body.into_boxed_slice())
    }
}

impl RleCompilable for ast::Program {
    fn with_ast<F, R>(&self, k: F) -> R
    where
        F: FnOnce(&ast::Program) -> R,
    {
        k(self)
    }
}
