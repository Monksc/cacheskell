use std::collections::HashMap;

use lang_parser::*;

use super::*;

fn convert_bool(parser: &lang_parser:: LanguageProduction) -> bool {
    parser.index == 0
}

fn convert_integer(parser : &lang_parser::LanguageProduction, current_int: i64) -> i64 {

    if parser.index != 0 {
        return current_int;
    }
    
    //assert(parser.name, String::from("integer"));
    //assert(parser.name, String::from("integerTail"));

    match (&parser.productions[0], &parser.productions[1]) {
        (LanguageProductionOrTerm::Prod(p1), LanguageProductionOrTerm::Prod(p2)) => {
            let digit = convert_digit(&p1);
            let current_int = current_int * 10 + (digit as i64);
            convert_integer(&p2, current_int)
        },
        (_, _) => current_int,
    }
}

fn convert_letter(parser : &lang_parser::LanguageProduction) -> char {
    match &parser.productions[0] {
        LanguageProductionOrTerm::Term(Terminal(t)) => t.chars().next().unwrap(),
        LanguageProductionOrTerm::Prod(p) => convert_letter(&p),
    }
}

fn convert_digit(parser : &lang_parser::LanguageProduction) -> u8 {
    //assert(parser.name, String::from("letter"));
    return parser.index as u8;
}

fn convert_identifier(parser : &lang_parser::LanguageProduction) -> String {

    //assert(parser.name, String::from("identifier"));
    //assert(parser.name, String::from("identifiertail"));

    let mut r = String::new();

    match (parser.index, &parser.productions[0])  {
        (i, LanguageProductionOrTerm::Prod(p)) => if i == 0 || i == 1 {
            r.push(convert_letter(&p));
            if let LanguageProductionOrTerm::Prod(p2) = &parser.productions[1] {
                let tail = convert_identifier(&p2);
                r.push_str(tail.as_str());
            }
        }
        (_, _) => return r,
    }

    return r
}

fn convert_variable(parser : &lang_parser::LanguageProduction) -> Expression {

    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[0] {
        match parser.index {
            0 => return Expression::Boolean(convert_bool(&p)),
            1 => return Expression::Int(convert_integer(&p, 0)),
            2 => return Expression::Variable(convert_identifier(&p)),
            _ => panic!(),
        }
    }

    panic!();
}

fn convert_functioncall_params(parser : &lang_parser::LanguageProduction, 
    params: &mut Vec<Expression>) {

    if parser.index != 0 {
        return;
    }

    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[0] {
        let expr = convert_expression(&p);
        params.push(expr);
    }


    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[2] {
        convert_functioncall_params(&p, params);
    }
}

fn convert_functioncall(parser : &lang_parser::LanguageProduction) -> FunctionCall {

    let function_name;
    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[2] {
        function_name = convert_identifier(&p);
    } else { panic!(); }

    let mut params = Vec::new();

    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[4] {
        convert_functioncall_params(&p, &mut params);
    }

    return FunctionCall::from(function_name, params);
}

fn convert_expression(parser : &lang_parser::LanguageProduction) -> Expression {
    if parser.index == 0 {
        if let LanguageProductionOrTerm::Prod(p) = &parser.productions[0] {
            return Expression::FuncCall(convert_functioncall(&p));
        }
    }

    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[0] {
        return convert_variable(&p);
    }

    panic!();
}

fn convert_bool_expression(parser : &lang_parser::LanguageProduction) -> Expression {
    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[0] {
        return convert_expression(&p);
    }
    panic!();
}

fn convert_case(parser : &lang_parser::LanguageProduction) -> Case {

    let bool_expr;
    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[2] {
        bool_expr = convert_bool_expression(&p);
    } else { panic!(); }

    let expr;
    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[6] {
        expr = convert_expression(&p);
    } else { panic!(); }

    return Case {
        bool_expr: bool_expr,
        return_expr : expr,
    };
}

fn convert_function_cases(parser : &lang_parser::LanguageProduction, cases : &mut Vec<Case>) {

    if parser.index != 0 {
        return;
    }

    let case;
    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[0] {
        case = convert_case(&p);
        cases.push(case);
    } else { panic!(); }

    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[2] {
        convert_function_cases(&p, cases);
    }
}

fn convert_function_params(parser : &lang_parser::LanguageProduction, params : &mut Vec<String>) {

    if parser.index != 0 {
        return;
    }

    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[0] {
        let identifier = convert_identifier(&p);
        params.push(identifier);
    }

    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[2] {
        convert_function_params(&p, params);
    }
}

fn convert_function(parser : &lang_parser::LanguageProduction) 
    -> (String, Function)  {

    //assert(parser.name, String::from("function"));

    let identifier;
    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[0] {
        identifier = convert_identifier(&p);
    } else { panic!(); }

    let mut params = Vec::new();
    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[2] {
        convert_function_params(&p, &mut params);
    } else { panic!(); }

    let mut cases = Vec::new();
    if let LanguageProductionOrTerm::Prod(p) = &parser.productions[3] {
        convert_function_cases(&p, &mut cases);
    } else { panic!(); }

    return (identifier, Function::from(cases, params));
}

fn convert_functions(parser : &lang_parser::LanguageProduction, 
    functions: &mut HashMap<String, Function>) {

    if parser.index != 0 {
        return;
    }

    if let lang_parser::LanguageProductionOrTerm::Prod(p) = &parser.productions[0] {
        let (name, function) = convert_function(&p);
        functions.insert(name, function);
    }

    if let lang_parser::LanguageProductionOrTerm::Prod(p) = &parser.productions[2] {
        convert_functions(&p, functions);
    }
}

pub fn convert_language(parser : &lang_parser::LanguageProduction) 
    -> HashMap<String, Function> {

    let mut functions = HashMap::new();
    if let lang_parser::LanguageProductionOrTerm::Prod(parser) = &parser.productions[1] {
        convert_functions(&parser, &mut functions);
    }

    return functions;

}
