mod lexer;
mod parser;
mod interpreter;
mod environment;

fn main() {
    // println!("Lexing");
    let mut lex: lexer::Lexer = lexer::Lexer::collect_dabs(std::fs::read_to_string("src/hello.cbjg").unwrap().as_str());
    // println!("Parsing");
    let mut parser: parser::Parser = parser::Parser::create_ast(&mut lex);
    // println!("Parsed! {:?}", parser.ast);
    // println!("Interpreting");
    let mut interprets: Vec<interpreter::RuntimeResult> = Vec::new();
    let mut last_interpreted: interpreter::RuntimeResult = interpreter::RuntimeResult::NullValue(interpreter::NullValue { value:"null".to_string() });
    let mut environments = environment::Environment::create_env();
    while parser.ast.len() > 0 { last_interpreted = interpreter::evaluate(parser.mov_expression(), &mut environments); interprets.push(last_interpreted); }
    // println!("Interpreted: {:?}", interprets);
}
