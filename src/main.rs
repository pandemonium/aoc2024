use std::{
    fmt::{self, Display},
    io::{self, Read},
};

use aoc2024::kombo::{
    char, enclosed_within, separated_by, string, such_that, ParseResult, ParseState, Parser,
};

#[derive(Clone, Debug)]
enum Expression {
    Apply {
        symbol: String,
        arguments: Vec<Literal>,
    },
}

#[derive(Clone, Debug)]
struct Literal(Constant);

#[derive(Clone, Debug)]
enum Constant {
    Integer(u128),
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
                .parse::<u128>()
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

fn expression() -> impl Parser<In = char, Out = Expression> {
    mul()
}

fn extract_expressions(source_text: &str) -> Vec<Expression> {
    let input = source_text.chars().collect::<Vec<_>>();
    let mut state = ParseState::new(&input);

    let mut expressions = vec![];

    loop {
        let result = expression().parse(state);

        if let Some(expression) = result.parsed {
            expressions.push(expression);
            state = result.state;
        } else {
            if result.state.can_advance(1) {
                state = result.state.advance(1);
            } else {
                break expressions;
            }
        }
    }
}

fn product(lits: Vec<Literal>) -> Literal {
    let x = lits
        .into_iter()
        .map(|Literal(Constant::Integer(x))| x)
        .product();

    Literal(Constant::Integer(x))
}

fn sum(lits: Vec<Literal>) -> Literal {
    let x = lits
        .into_iter()
        .map(|Literal(Constant::Integer(x))| x)
        .sum();

    Literal(Constant::Integer(x))
}

fn evaluate(expr: Expression) -> Literal {
    match expr {
        Expression::Apply { arguments, .. } => product(arguments),
    }
}

fn run_for_input<'a>(input: &str) {
    let exprs = extract_expressions(input);
    //    for expr in &exprs {
    //        println!("{}", expr);
    //    }
    //
    //    println!("Count: {}", exprs.len());

    let lits = extract_expressions(input)
        .into_iter()
        .map(evaluate)
        .collect();

    let x = sum(lits);
    println!("{:?}", x);
}

fn main() {
    let mut buf = String::with_capacity(1024);
    io::stdin().read_to_string(&mut buf).unwrap();

    let answer = run_for_input(&buf);

    //    println!("Answer: {}", answer);
}
