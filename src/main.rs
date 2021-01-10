use std::env;
use std::fs;
use std::collections::HashMap;

use lang_parser::*;
use lambda_lang::*;

fn equal(expressions: &Vec<Expression>) -> Expression {
    if expressions.len() == 2 {
        match (&expressions[0], &expressions[1]) {
            (Expression::Int(x1), Expression::Int(x2)) => return Expression::Boolean(x1 == x2),
            (_, _) => return Expression::Boolean(false),
        }
    }

    return Expression::Boolean(false);
}

fn add(expressions: &Vec<Expression>) -> Expression {
    if expressions.len() == 2 {
        match (&expressions[0], &expressions[1]) {
            (Expression::Int(x1), Expression::Int(x2)) => return Expression::Int(x1 + x2),
            (x1, x2) => println!("{:#?}\n{:#?}", x1, x2),
        }
    }

    return Expression::Int(69);
}

fn sub(expressions: &Vec<Expression>) -> Expression {
    if expressions.len() == 2 {
        match (&expressions[0], &expressions[1]) {
            (Expression::Int(x1), Expression::Int(x2)) => return Expression::Int(x1 - x2),
            (x1, x2) => println!("{:#?}\n{:#?}", x1, x2),
        }
    }

    return Expression::Int(69);
}

fn print(expressions: &Vec<Expression>) -> Expression {

    for expr in expressions {
        match expr {
            Expression::FuncCall(c) => println!("{:#?}", c),
            Expression::Variable(v)     => println!("{:#?}", v),
            Expression::Boolean(b)      => println!("{:#?}", b),
            Expression::Int(i)          => println!("{:#?}", i),
            Expression::Void            => println!("VOID"),
        }
    }

    return Expression::Boolean(false);
}


fn run_fib() {
    println!("Hello, world!");
    println!("YEET!");

    let fib = Function::from(
        vec![
            Case {
                bool_expr: Expression::FuncCall(
                    FunctionCall::from(
                        String::from("equal"), 
                        vec![
                            Expression::Variable(String::from("x")),
                            Expression::Int(0),
                        ]
                    )
                ),
                return_expr: Expression::Int(0)
            },
            Case {
                bool_expr: Expression::FuncCall(
                    FunctionCall::from(
                        String::from("equal"), 
                        vec![
                            Expression::Variable(String::from("x")),
                            Expression::Int(1),
                        ]
                    )
                ),
                return_expr: Expression::Int(1)
            },
            Case {
                bool_expr: Expression::FuncCall(
                    FunctionCall::from(
                        String::from("equal"), 
                        vec![
                            Expression::Variable(String::from("x")),
                            Expression::Int(2),
                        ]
                    )
                ),
                return_expr: Expression::Int(1)
            },
            Case {
                bool_expr: Expression::Boolean(true),
                return_expr: Expression::FuncCall(
                    FunctionCall::from(
                        String::from("add"),
                        vec![
                            Expression::FuncCall(
                                FunctionCall::from(
                                    String::from("fib"),
                                    vec![
                                        Expression::FuncCall(
                                            FunctionCall::from(
                                                String::from("add"),
                                                vec![
                                                    Expression::Variable(
                                                        String::from("x")
                                                    ),
                                                    Expression::Int(-1),
                                                ]
                                            )
                                        )
                                    ]
                                )
                            ),
                            Expression::FuncCall(
                                FunctionCall::from(
                                    String::from("fib"),
                                    vec![
                                        Expression::FuncCall(
                                            FunctionCall::from(
                                                String::from("add"),
                                                vec![
                                                    Expression::Variable(
                                                        String::from("x")
                                                    ),
                                                    Expression::Int(-2),
                                                ]
                                            )
                                        )
                                    ]
                                )
                            ),
                        ]
                    ),
                ),
            },
        ],
        vec![
            String::from("x"),
        ],
    );


    let mut functions : HashMap<String, fn(&Vec<Expression>) -> Expression> = HashMap::new();
    functions.insert(String::from("equal"), equal);
    functions.insert(String::from("add"), add);

    let mut inter = lambda_lang::Interpreter::from(
        vec![("fib".to_string(), fib)]
            .into_iter()
            .collect(),
            functions
    );

    let result = inter.reduce_expression(
        &Expression::FuncCall(
            FunctionCall::from(
                String::from("fib"),
                vec![
                    Expression::Int(50),
                ]
            )
        )
    );

    println!("{:#?}", result);
}

fn main() {
    //run_fib();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("You need to supply a .chs file. Example: cacheskell <.chs file>");
    }

    let codefile = "/Users/cameronmonks/Projects/rust/lambda_lang/languagefiles/cacheskell.ll";

    let contents = fs::read_to_string(codefile)
        .expect("Something went wrong reading the file");

    let mut interp = lang_parser::Interpreter::new();
    interp.add_interpreter(&contents);

    let file_name = &args[1];
    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file given.");

    let parse;
    if let Ok(p) = interp.parse(&"program".to_string(), &contents) {
        parse = p;
    } else {
        panic!();
    }

    //println!("{:#?}", parse);
    
    // Convert code

    let convertedcode_functions = convert_language(&parse);

    
    // Run Code

    let mut functions : HashMap<String, fn(&Vec<Expression>) -> Expression> = HashMap::new();
    functions.insert(String::from("equal"), equal);
    functions.insert(String::from("add"), add);
    functions.insert(String::from("sub"), sub);
    functions.insert(String::from("print"), print);

    let mut inter = lambda_lang::Interpreter::from(convertedcode_functions, functions);

    let _ = inter.reduce_expression(
        &Expression::FuncCall(
            FunctionCall::from(
                String::from("main"),
                vec![]
            )
        )
    );

}

