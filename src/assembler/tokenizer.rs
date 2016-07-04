//
//  Assembler grammmar
//
//  program := { line, "\n" }
//  line := instruction | lebel | comment | ""
//  
//  label := "(", identifier, ")"
//  identifier := sequence of letters, number, and underscores s.t. the first character is a letter
//
//  instruction := a-instruction | c-instruction
//  a-instruction := "@", identifier | literal
//  literal := 15 bit unsigned int, (0 <= n < 2**15)
//
//  c-instruction := [ [ dest, "=" ], comp ], [ ";" jmp ]
//  dest := /r/A?M?D?/g
//  cop := a recognized computation
//  jmp := "JLT" | "JLE" | "JEQ" | "JNE" | "JGT" | "JGE" | "JMP"
//
//



enum TokenType { symbol, literal, lparen, rparen, assign_op, jmp_sep, loc_op, jump, dest, comp, comment };

struct Token {
    ttype: TokenType,
    text: &str
}


struct Tokenizer {
    line: &str,
    pos: usize,
    len: usize,
    lineno: i32,
    doc: &str
}

impl Tokenizer {

    fn new(&str body) -> Tokenizer {
        Tokenizer { body: body, pos: 0 as usize }
    }

    fn next_token(&mut self) -> Token {}



fn assemble(raw: &'static str) -> Vec<u16> {
    let symbol_table = init_symbol_table();
    let instructions = Vec<u16>::new();

    for line in raw.split("\n") {
    }

}
