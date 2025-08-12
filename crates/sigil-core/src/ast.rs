#[derive(Debug, Clone)]
pub enum Stmt {
    FunDecl(String, Vec<String>, Box<Stmt>),
    Return(Expr),
    If(Expr, Box<Stmt>),
    Block(Vec<Stmt>),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Variable(String),
    Call(String, Vec<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
}
