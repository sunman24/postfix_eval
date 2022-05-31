mod tests;

use std::collections::HashSet;

#[allow(unused)]
pub(crate) struct PostfixEvaluator {
    operators_set: HashSet<&'static str>,
    operand_stack: Vec<f64>,
}

#[allow(unused)]
impl PostfixEvaluator {    
    pub fn new() -> Self {
        let operators_set = Self::get_operators_set();
        let operand_stack: Vec<f64> = vec![];

        Self {
            operators_set,
            operand_stack
        }
    }

    // Evaluates the expression in the postfix form of the entry
    pub fn eval(&mut self, postfix_expr: &str) -> Result<f64, String> {
        let tokens = postfix_expr.split_whitespace();

        for token in tokens {
            let parse_result = token.parse::<f64>();
            
            match parse_result {
                Ok(val) => {
                    self.operand_stack.push(val);
                    continue;
                },

                Err(_) => {
                    if !self.operators_set.contains(token) {
                        return Err(format!("unknown token '{token}'"));
                    }
                },
            }

            let operator = token;
            
            match self.bin_op_action_matcher(operator) {
                Ok(_) => {},
                Err(msg) => {
                    return Err(msg);
                }
            }
        }
        
        self.operand_stack.pop().ok_or("operand not found".to_string())
    }

    // Calculates and push the result of a binary operator to operand stack
    #[inline]
    fn bin_op_action_matcher(&mut self, operator: &str) -> Result<(), String> {
        let operand2 = self.operand_stack.pop();
        let operand1 = self.operand_stack.pop();

        if (operand1 == None) || (operand2 == None) {
            let operand_count = 
                operand1.map_or(0, |_| 1) +
                operand2.map_or(0, |_| 1);

            return Err(format!("the '{operator}' operator requires two operands, but {operand_count} was found"));
        }

        let operand1 = operand1.unwrap();
        let operand2 = operand2.unwrap();
        let result = match operator {
            "+" => Ok(operand1 + operand2),
            "-" => Ok(operand1 - operand2),
            "*" => Ok(operand1 * operand2),
            "/" => Ok(operand1 / operand2),
            _ => Err(format!("unknown operator '{operator}'")),
        };

        match result {
            Ok(val) => Ok(self.operand_stack.push(val)),
            Err(msg) => {
                return Err(msg);
            },
        }
    }

    fn get_operators_set() -> HashSet<&'static str> {
        let mut set: HashSet<&'static str> = HashSet::new();
    
        set.insert("+");
        set.insert("-");
        set.insert("*");
        set.insert("/");
    
        set
    }
}
