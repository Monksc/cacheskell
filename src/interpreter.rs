use std::collections::HashMap;

use super::*;


pub struct Interpreter {
    function_map : HashMap<String, Function>,
    preset_functions: HashMap<String, fn (&Vec<Expression>) -> Expression>,
    cache: FunctionCache,
}

impl Interpreter {

    pub fn new() -> Interpreter {
        Interpreter { 
            function_map: HashMap::new(), 
            preset_functions: HashMap::new(),
            cache: FunctionCache::new(512),
        }
    }

    pub fn from(functions: HashMap<String, Function>, 
        preset_functions: HashMap<String, fn(&Vec<Expression>) -> Expression>) -> Interpreter {

        Interpreter { 
            function_map: functions,
            preset_functions: preset_functions,
            cache: FunctionCache::new(512),
        }
    }


    // MARK: Applicative Order

    fn applicative_form_functioncall(&mut self, call : &FunctionCall) -> Expression {

        let params = &call.get_parameters()
                    .iter()
                    .map(|x| self.reduce_expression(&x))
                    .collect::<Vec<_>>();

        if !self.function_map.contains_key(call.get_name()) {
            match self.preset_functions.get(call.get_name()) {
                Some(f) => return f(params),
                None => return Expression::Boolean(false),
            }
        }
        

        let func = self.function_map[call.get_name()].clone();

        let call = &FunctionCall::from(call.get_name().clone(), params.clone());

        if params.len() < func.get_variables().len() {
            return Expression::FuncCall(call.clone());
        } else if params.len() < func.get_variables().len() {
            return Expression::Boolean(false);
        }

        // Replace

        let mut replace_var = HashMap::new();
        for i in 0..func.get_variables().len() {
            replace_var.insert(
                func.get_variables()[i].clone(), params[i].clone());
        }

        // Check if solve
        
        if let Some(answer) = self.cache.get(call) {
            return answer;
        }
        
        
        // Solve
        
        for case in func.get_cases() {
            
            let expr = self.reduce_expression(&case.bool_expr.replace_variables(&replace_var));

            if let Expression::Boolean(b) = expr {
                if b {
                    let expr = self.reduce_expression(
                        &case.return_expr.replace_variables(&replace_var));
                    self.cache.put(call.clone(), expr.clone());
                    return expr;
                }
            }
        }

        return Expression::FuncCall(call.clone());
    }


    // MARK: Normal Order
    
    fn normal_form_functioncall(&mut self, call : &FunctionCall) -> Expression {

        if !self.function_map.contains_key(call.get_name()) {
            match self.preset_functions.get(call.get_name()) {
                Some(f) => return f(&call.get_parameters()
                    .iter()
                    .map(|x| self.reduce_expression(&x))
                    .collect::<Vec<_>>()),
                None => return Expression::Boolean(false),
            }
        }


        let func = self.function_map[call.get_name()].clone();

        if call.get_parameters().len() != func.get_variables().len() {
            return Expression::FuncCall(call.clone());
        }

        let mut replace_var = HashMap::new();
        for i in 0..func.get_variables().len() {
            replace_var.insert(
                func.get_variables()[i].clone(), call.get_parameters()[i].clone());
        }
        
        for case in func.get_cases() {
            
            let expr = self.reduce_expression(&case.bool_expr.replace_variables(&replace_var));

            if let Expression::Boolean(b) = expr {
                if b {
                    return self.reduce_expression(
                        &case.return_expr.replace_variables(&replace_var));
                }
            }
        }

        return Expression::FuncCall(call.clone());
    }

    pub fn reduce_expression(&mut self, 
        expression: &Expression) -> Expression {

        if let Expression::FuncCall(c) = expression {
            //return self.normal_form_functioncall(c);   
            return self.applicative_form_functioncall(c);   
        } else {
            return expression.clone();
        }
    }

}

