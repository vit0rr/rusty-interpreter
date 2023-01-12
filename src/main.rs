use std::collections::HashMap;

struct Let {
    param: String,
    value: Box<Ast>,
    next: Box<Ast>,
}

impl Let {
    fn new(param: String, value: Box<Ast>, next: Box<Ast>) -> Self {
        Self { param, value, next }
    }
}

struct Number {
    value: i32,
}

impl Number {
    fn new(value: i32) -> Self {
        Self { value }
    }
}

struct Add {
    left: Box<Ast>,
    right: Box<Ast>,
}

impl Add {
    fn new(left: Box<Ast>, right: Box<Ast>) -> Self {
        Self { left, right }
    }
}

struct If {
    condition: Box<Ast>,
    then: Box<Ast>,
    else_: Box<Ast>,
}

impl If {
    fn new(condition: Box<Ast>, then: Box<Ast>, else_: Box<Ast>) -> Self {
        Self {
            condition,
            then,
            else_,
        }
    }
}

struct Id {
    value: String,
}

impl Id {
    fn new(value: String) -> Self {
        Self { value }
    }
}

enum Ast {
    Let(Let),
    Number(Number),
    Add(Add),
    If(If),
    Id(Id),
}

fn interpret(environment: &mut HashMap<String, i32>, ast: &Ast) -> i32 {
    match ast {
        Ast::Let(Let { param, value, next }) => {
            let result = interpret(environment, value);
            environment.insert(param.to_string(), result);
            interpret(environment, next)
        }
        Ast::Number(Number { value }) => *value,
        Ast::Add(Add { left, right }) => {
            interpret(environment, left) + interpret(environment, right)
        }
        Ast::If(If {
            condition,
            then,
            else_,
        }) => {
            if interpret(environment, condition) != 0 {
                interpret(environment, then)
            } else {
                interpret(environment, else_)
            }
        }
        Ast::Id(Id { value }) => *environment.get(value).unwrap_or(&0),
    }
}

fn main() {
    let mut environment: HashMap<String, i32> = HashMap::new();
    let sum_test = Ast::Let(Let::new(
        "a".to_string(),
        Box::new(Ast::Number(Number::new(2))),
        Box::new(Ast::Let(Let::new(
            "b".to_string(),
            Box::new(Ast::Number(Number::new(3))),
            Box::new(Ast::Add(Add::new(
                Box::new(Ast::Id(Id::new("a".to_string()))),
                Box::new(Ast::Id(Id::new("b".to_string()))),
            ))),
        ))),
    ));

    let if_test = Ast::If(If::new(
        Box::new(Ast::Number(Number::new(true as i32))),
        Box::new(Ast::Number(Number::new(1))),
        Box::new(Ast::Number(Number::new(0))),
    ));

    println!("sumTest: {}", interpret(&mut environment, &sum_test));
    println!("ifTest: {}", interpret(&mut environment, &if_test));
}
