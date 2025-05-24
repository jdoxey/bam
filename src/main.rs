use lalrpop_util::lalrpop_mod;

#[derive(Debug)]
pub enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
}

lalrpop_mod!(pub grammar);

fn main() {
    let parser = grammar::ExprParser::new();
    let expr_str = "1+2+3";
    let ast: Expr = parser.parse(expr_str).unwrap();
    println!("{:#?}", ast);
}
