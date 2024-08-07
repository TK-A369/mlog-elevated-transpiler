use crate::parser::*;

impl ProgramAST {
    pub fn generate(&self) -> String {
        let mut result_code = String::new();
        let mut uid: usize = 0;
        result_code
    }
}

impl FunctionAST {
    fn generate(&self, global_variables: &[GlobalVariableAST], uid: &mut usize) -> String {
        let mut result_code = String::new();
        let mut local_variables = Vec::<LocalVariableAST>::new();
        for statement in &self.statements {
            statement.generate(
                global_variables,
                &mut local_variables,
                &mut result_code,
                uid,
            );
        }
        result_code
    }
}

impl StatementASTNode {
    fn generate(
        &self,
        global_variables: &[GlobalVariableAST],
        local_variables: &mut Vec<LocalVariableAST>,
        result_code: &mut String,
        uid: &mut usize,
    ) {
        match self {
            StatementASTNode::LocalVariableAST(lvs @ LocalVariableAST { name: lv }) => {
                local_variables.push(lvs.clone());
            }
            StatementASTNode::AssignmentAST(AssignmentAST {
                target_var_name,
                value,
            }) => {
                todo!();
                //match value {}
            }
            StatementASTNode::ExpressionAST(expr) => match expr {
                ExpressionASTNode::FunctionCallAST(FunctionCallAST {
                    function_name,
                    args,
                }) => {}
                _ => {
                    //Using string or number literal or variable reference as statement is noop
                }
            },
            StatementASTNode::IfAST(IfAST {
                condition,
                then_block,
                else_block,
            }) => {
                let else_label = format!("else_{}", uid);
                *uid += 1;
                let condition_buf = format!("cond_{}", uid);
                *uid += 1;

                let assign_condition_statement = StatementASTNode::AssignmentAST(AssignmentAST {
                    target_var_name: condition_buf.clone(),
                    value: condition.clone(),
                });
                assign_condition_statement.generate(
                    global_variables,
                    local_variables,
                    result_code,
                    uid,
                );

                result_code.push_str(&format!("jump {} equal {} 0\n", else_label, condition_buf));
                for then_statement in then_block {
                    then_statement.generate(global_variables, local_variables, result_code, uid);
                }

                result_code.push_str(&else_label);
                result_code.push_str(":\n");
                for else_statement in else_block {
                    else_statement.generate(global_variables, local_variables, result_code, uid);
                }
            }
            StatementASTNode::WhileAST(WhileAST {
                condition,
                do_block,
            }) => {
                let while_begin_label = format!("while_begin_{}", uid);
                *uid += 1;
                let while_end_label = format!("while_end_{}", uid);
                *uid += 1;
                let condition_buf = format!("cond_{}", uid);
                *uid += 1;

                let assign_condition_statement = StatementASTNode::AssignmentAST(AssignmentAST {
                    target_var_name: condition_buf.clone(),
                    value: condition.clone(),
                });

                result_code.push_str(&while_begin_label);
                result_code.push_str(&format!(
                    ":\njump {} equal {} 0\n",
                    while_end_label, condition_buf
                ));

                for do_statement in do_block {
                    do_statement.generate(global_variables, local_variables, result_code, uid);
                }

                result_code.push_str(&format!("jump {} always", while_begin_label));

                result_code.push_str(&while_end_label);
                result_code.push_str(":\n");
            }
        }
    }
}

impl FunctionCallAST {
    fn generate(&self) {}
}
