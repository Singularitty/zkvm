use chumsky::prelude::*;

use crate::asm::asm::*;

// Type Aliases
type I<'a> = &'a str;
type E<'a> = extra::Err<Rich<'a, char>>;

//
// -- Parsers --
//

// Identifiers for labels: [A-Za-z_][A-Za-z0-9_]*
fn ident<'a>() -> impl Parser<'a, I<'a>, String, E<'a>> {
    text::ascii::ident().map(|s: &str| s.to_string())
}

// Signned Ints with hexadecimal
fn int32<'a>() -> impl Parser<'a, I<'a>, i32, E<'a>> {
    let sign = just('-').or(just('+')).or_not().to_slice();

    let decimal = text::digits(10).to_slice().from_str::<i32>().unwrapped();
    let hex = just("0x")
        .ignore_then(text::digits(16).to_slice())
        .try_map(|s: &str, span| i32::from_str_radix(s, 16).map_err(|_| Rich::custom(span, "bad hex")));

    sign.then(choice((hex, decimal))).map(|(sgn, n)| if sgn == "-" {-n} else {n})
}

// Registers
fn register<'a>() -> impl Parser<'a, I<'a>, Register, E<'a>> {
    choice((
        text::keyword("X0").to(Register::X0),
        text::keyword("X1").to(Register::X1),
        text::keyword("X2").to(Register::X2),
        text::keyword("X3").to(Register::X3),
        text::keyword("X4").to(Register::X4),
        text::keyword("X5").to(Register::X5),
        text::keyword("X6").to(Register::X6),
        text::keyword("X7").to(Register::X7),
    ))
}

// Operands
fn operand<'a>() -> impl Parser<'a, I<'a>, Operand, E<'a>> {
    choice((
        register().map(Operand::Register),
        int32().map(Operand::Immediate),
        ident().map(Operand::Label),
    ))
    .then_ignore(sp0())
}

// Comma char
fn comma<'a>() -> impl Parser<'a, I<'a>, (), E<'a>> {
    just(',').then_ignore(sp0()).ignored()
}

// Instructions

fn add_instr<'a>() -> impl Parser<'a, I<'a>, Instruction, E<'a>> {
    text::keyword("ADD")
        .then_ignore(sp0())
        .ignore_then(
            operand()
                .then_ignore(comma())
                .then(operand())
                .then_ignore(comma())
                .then(operand()),
        )
        .map(|((a,b), c)| Instruction::ADD(a, b, c))
}

fn addi_instr<'a>() -> impl Parser<'a, I<'a>, Instruction, E<'a>> {
    text::keyword("ADDI")
        .then_ignore(sp0())
        .ignore_then(
            operand()
                .then_ignore(comma())
                .then(operand())
                .then_ignore(comma())
                .then(operand()),
        )
        .map(|((a,b), c)| Instruction::ADDI(a, b, c))
}

fn mov_instr<'a>() -> impl Parser<'a, I<'a>, Instruction, E<'a>> {
    text::keyword("MOV")
        .then_ignore(sp0())
        .ignore_then(operand().then_ignore(comma()).then(operand()))
        .map(|(dst, src)| Instruction::MOV(dst, src))
}

fn jmp_instr<'a>() -> impl Parser<'a, I<'a>, Instruction, E<'a>> {
    text::keyword("JMP")
        .then_ignore(sp0())
        .ignore_then(operand())
        .map(Instruction::JMP)
}

fn jz_instr<'a>() -> impl Parser<'a, I<'a>, Instruction, E<'a>> {
    text::keyword("JZ")
        .then_ignore(sp0())
        .ignore_then(operand().then_ignore(comma()).then(operand()))
        .map(|(cond, target)| Instruction::JZ(cond, target))
}

fn halt_instr<'a>() -> impl Parser<'a, I<'a>, Instruction, E<'a>> {
    text::keyword("HALT").then_ignore(sp0()).to(Instruction::HALT)
}

fn instr<'a>() -> impl Parser<'a, I<'a>, Instruction, E<'a>> {
    choice((
        add_instr(),
        addi_instr(),
        mov_instr(),
        jmp_instr(),
        jz_instr(),
        halt_instr(),
    ))
}

fn label_decl<'a>() -> impl Parser<'a, I<'a>, String, E<'a>> {
    ident().then_ignore(just(':'))
}

fn comment<'a>() -> impl Parser<'a, I<'a>, (), E<'a>> {
    let newlines = "\n\r\x0B\x0C\u{0085}\u{2028}\u{2029}";
    just(';').ignore_then(none_of(newlines).repeated()).ignored()
}

fn stmt<'a>() -> impl Parser<'a, I<'a>, Stmt, E<'a>> {
    choice((
        label_decl().map(Stmt::Label),
        instr().map(Stmt::Instr)
    ))
    .then_ignore(sp0())
}

fn ws1<'a>() -> impl Parser<'a, I<'a>, (), E<'a>> {
        one_of(" \t")
        .repeated()        // build the repetition
        .at_least(1)       // require ≥ 1
        .ignored()
}

// Spaces/tabs (no newlines)
fn sp0<'a>() -> impl Parser<'a, I<'a>, (), E<'a>> {
    one_of(" \t").repeated().ignored()
}

// indentation at line start
fn indent<'a>() -> impl Parser<'a, I<'a>, (), E<'a>> {
    sp0()
}

pub fn program<'a>() -> impl Parser<'a, I<'a>, Program, E<'a>> {
    // [indent] stmt [spaces]* [; comment]? then newline OR EOF
    let stmt_line = sp0()
        .ignore_then(
            stmt()
                .map(Some)
                .then_ignore(sp0())
                .then_ignore(comment().or_not()),
        )
        .then_ignore(text::newline().or(end()))
        .labelled("statement-line");

    // [indent] ; ... then newline OR EOF
    let comment_line = sp0()
        .ignore_then(comment())
        .then_ignore(text::newline().or(end()))
        .to::<Option<Stmt>>(None)
        .labelled("comment-line");

    // [indent] newline  (MUST be a real newline; not EOF)
    let blank_line = sp0()
        .then_ignore(text::newline())
        .to::<Option<Stmt>>(None)
        .labelled("blank-line");

    // choice of the three; each variant consumes its own terminator
    let line = choice((stmt_line, comment_line, blank_line));

    // many lines; EOF is already handled by stmt/comment lines,
    // so we don't also call `end()` here.
    line.repeated()
        .collect::<Vec<Option<Stmt>>>()
        .map_with(|lines, _| {
            let stmts: Vec<Stmt> = lines.into_iter().flatten().collect();
            Program(stmts)
        })
}