use crate::parser::*;

impl ProgramAST {
    fn generate(&self) -> String {
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
                result_code
                    .push_str(format!("jump {} equal {} 0", else_label, condition_buf).as_str());
            }
            StatementASTNode::WhileAST(WhileAST {
                condition,
                do_block,
            }) => {
                let while_label = format!("while_{}", uid);
                *uid += 1;
                let while_end = format!("while_end_{}", uid);
                *uid += 1;
            }
        }
    }
}

impl FunctionCallAST {
    fn generate(&self) {}
}
