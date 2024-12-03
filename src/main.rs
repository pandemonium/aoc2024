use std::{
    fmt::{self, Display},
    io::{self, Read},
};

use aoc2024::kombo::{char, enclosed_within, separated_by, string, such_that, ParseState, Parser};

#[derive(Clone, Debug)]
enum Expression {
    Apply {
        symbol: String,
        arguments: Vec<Literal>,
    },
}

#[derive(Clone, Debug)]
struct Statement(Expression);

#[derive(Clone, Debug)]
struct Literal(Constant);

#[derive(Clone, Debug)]
enum Constant {
    Integer(u64),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let Self(Constant::Integer(x)) = self;
        write!(f, "{x}")
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Apply { symbol, arguments } => {
                write!(f, "{symbol}(")?;
                let args = arguments
                    .iter()
                    .map(|x| format!("{x}"))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{args})")
            }
        }
    }
}

fn digit() -> impl Parser<In = char, Out = char> {
    such_that(|c| char::is_digit(*c, 10))
}

fn integer() -> impl Parser<In = char, Out = Literal> {
    digit()
        .one_or_more()
        //        .filter_map(|image| if image.len() <= 3 { Some(image) } else { None })
        .map(|image| {
            image
                .into_iter()
                .collect::<String>()
                .parse::<u64>()
                .ok()
                .map(Constant::Integer)
                .map(Literal)
        })
        .filter_map(|x| x)
}

fn mul() -> impl Parser<In = char, Out = Expression> {
    let args = enclosed_within(char('('), char(')'), separated_by(integer(), char(',')));

    string("mul")
        .skip_left(args)
        .filter_map(|xs| if xs.len() != 2 { None } else { Some(xs) })
        .map(|arguments| Expression::Apply {
            symbol: "mul".to_owned(),
            arguments,
        })
}

fn enable_mul() -> impl Parser<In = char, Out = Expression> {
    string("do()").map(|_| Expression::Apply {
        symbol: "do".to_owned(),
        arguments: vec![],
    })
}

fn disable_mul() -> impl Parser<In = char, Out = Expression> {
    string("don't()").map(|_| Expression::Apply {
        symbol: "don't".to_owned(),
        arguments: vec![],
    })
}

fn statement() -> impl Parser<In = char, Out = Statement> {
    disable_mul()
        .or_else(enable_mul())
        .or_else(mul())
        .map(Statement)
}

fn parse_program(source_text: &str) -> Vec<Statement> {
    let input = source_text.chars().collect::<Vec<_>>();
    let mut state = ParseState::new(&input);

    let mut statements = vec![];

    loop {
        let result = statement().parse(state);

        if let Some(statement) = result.parsed {
            statements.push(statement);
            state = result.state;
        } else {
            if result.state.can_advance(1) {
                state = result.state.advance(1);
            } else {
                break statements;
            }
        }
    }
}

fn product(lits: &[Literal]) -> u64 {
    lits.into_iter()
        .map(|Literal(Constant::Integer(x))| x)
        .product()
}

fn sum(lits: &[Literal]) -> u64 {
    lits.into_iter()
        .map(|Literal(Constant::Integer(x))| x)
        .sum()
}

fn evaluate(expr: &Expression) -> u64 {
    match expr {
        Expression::Apply { arguments, .. } => product(arguments),
    }
}

struct AbstractMachine {
    mul_enable: bool,
    sum: u64,
}

impl AbstractMachine {
    fn new() -> Self {
        Self {
            mul_enable: true,
            sum: 0,
        }
    }

    fn interpret(&mut self, program: &[Statement]) {
        for Statement(expr) in program {
            match expr {
                Expression::Apply { symbol, .. } if symbol == "mul" && self.mul_enable => {
                    self.sum += evaluate(expr)
                }
                Expression::Apply { symbol, .. } if symbol == "do" => self.mul_enable = true,
                Expression::Apply { symbol, .. } if symbol == "don't" => self.mul_enable = false,
                _otherwise => (),
            }
        }
    }
}

fn run_for_input<'a>(input: &str) {
    let program = parse_program(input);

    let mut machine = AbstractMachine::new();
    machine.interpret(&program);
    println!("{:?}", machine.sum);
}

fn main() {
    let mut buf = String::with_capacity(1024);
    io::stdin().read_to_string(&mut buf).unwrap();

    let answer = run_for_input(&buf);

    //    println!("Answer: {}", answer);
}
