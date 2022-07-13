use std::collections::HashMap;

use chumsky::prelude::*;

type Program = Vec<Instr>;

type Registers = HashMap<String, i32>;

/// The Program output contains the registers and the highest register value seen during the
/// program run
type ProgramOutput = (Registers, i32);

pub fn parse_program(input: &str) -> Program {
    let parser = instr_parser();

    input
        .lines()
        .map(|line| parser.parse(line).unwrap())
        .collect()
}

pub fn run_program(program: &Program) -> ProgramOutput {
    let mut registers = Registers::new();
    let mut max = 0;

    for Instr { register, op, cond } in program.iter() {
        if cond.evaluate_cond(&registers) {
            let register = registers.entry(register.to_owned()).or_insert(0);

            match op {
                Op::Inc(v) => *register += v,
                Op::Dec(v) => *register -= v,
            }

            max = std::cmp::max(max, *register);
        }
    }

    (registers, max)
}

#[derive(Debug)]
pub struct Instr {
    register: String,
    op: Op,
    cond: Cond,
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Inc(i32),
    Dec(i32),
}

#[derive(Debug, Clone)]
struct Cond {
    register: String,
    op: CompOp,
    value: i32,
}

impl Cond {
    fn evaluate_cond(&self, registers: &Registers) -> bool {
        use CompOp::*;
        let reg_value = registers.get(&self.register).copied().unwrap_or(0);
        match (reg_value, self.value, self.op) {
            (l, r, GEq) => l >= r,
            (l, r, G) => l > r,
            (l, r, Eq) => l == r,
            (l, r, NEq) => l != r,
            (l, r, LEq) => l <= r,
            (l, r, L) => l < r,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum CompOp {
    GEq,
    G,
    Eq,
    NEq,
    LEq,
    L,
}

// Example instr:
// koq inc 675 if xrh >= -6
pub fn instr_parser() -> impl Parser<char, Instr, Error = Simple<char>> {
    let register = text::ident::<_, Simple<char>>().padded();

    let op = text::ident()
        .padded()
        .then(
            just('-')
                .or_not()
                .chain::<char, _, _>(text::int::<_, Simple<char>>(10))
                .collect::<String>()
                .try_map(|num, span| {
                    num.parse::<i32>()
                        .map_err(|e| Simple::custom(span, format!("{}", e)))
                }),
        )
        .try_map(|(op, num), span| match op.as_str() {
            "inc" => Ok(Op::Inc(num)),
            "dec" => Ok(Op::Dec(num)),
            _ => Err(Simple::custom(span, "Op must be inc or dec")),
        });

    let cond = just("if")
        .padded()
        .ignore_then(register)
        .then(
            just(">=")
                .to(CompOp::GEq)
                .or(just(">").to(CompOp::G))
                .or(just("==").to(CompOp::Eq))
                .or(just("!=").to(CompOp::NEq))
                .or(just("<=").to(CompOp::LEq))
                .or(just("<").to(CompOp::L))
                .padded(),
        )
        .then(
            just('-')
                .or_not()
                .chain::<char, _, _>(text::int(10))
                .collect::<String>()
                .try_map(|num, span| {
                    num.parse::<i32>()
                        .map_err(|e| Simple::custom(span, format!("{}", e)))
                }),
        )
        .map(|((register, op), value)| Cond {
            register,
            op,
            value,
        });

    register
        .then(op)
        .then(cond)
        .map(|((register, op), cond)| Instr { register, op, cond })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "\
            b inc 5 if a > 1
            a inc 1 if b < 5
            c dec -10 if a >= 1
            c inc -20 if c == 10";

        let program = parse_program(input);
        let (registers, max_held) = run_program(&program);

        let max = registers.into_iter().map(|(_, v)| v).max().unwrap();

        assert_eq!(1, max);
        assert_eq!(10, max_held);
    }
}
