mod lexer;
mod parser;

fn main() {
    println!(
        "{:?}",
        lexer::lexer::lex(
            "
x = 4;
foo = bar() + \"eat it\"
baz = 'bim bap\'bop\''; 2+2+2
"
        )
    );
}
