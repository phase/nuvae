mod ast;
mod parser;
mod diagnostic;

fn main() {
    let test_file = "test.nuv";
    let test_source = "\
    fun test(a: {v: Int32 | v >= 0 && v <= 10}, b): Int32 {
        let x = 1;
        let y = 0;
        if x < a {
            y = 7;
        } else if x >= 500 {
            y = 5;
        } else if x >= a && x < b {
            y = 9;
        } else {
           y = b;
        }
        return y;
    }".to_string();
    let mut parser = parser::Parser::new();
    let parsed_program = parser.parse(ast::Path::of("test"), test_file.to_string(), test_source);
    parser.diagnostics.emit_errors();
    if let Some((statement_arena, expression_arena, program)) = parsed_program {
        dbg!(program);
        dbg!(statement_arena);
        dbg!(expression_arena);
    }
}
