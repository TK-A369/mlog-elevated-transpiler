use crate::parser::*;

struct VariableScope {
    variables: Vec<LocalVariableAST>,
    mangle: String,
}

fn mangle_variable(
    variable_name: &str,
    global_variables: &std::collections::BTreeMap<String, GlobalVariableAST>,
    local_variables: &[VariableScope],
) -> Option<String> {
    for lvs in local_variables.iter().rev() {
        for lv in &lvs.variables {
            if lv.name == variable_name {
                let mut mangled_name = String::from(variable_name);
                mangled_name.push_str(&lvs.mangle);
                return Some(mangled_name);
            }
        }
    }

    if let Some(gv) = global_variables.get(variable_name) {
        return Some(variable_name.into());
    } else {
        return None;
    }
}

impl VariableScope {
    fn new(mangle: &str) -> Self {
        Self {
            variables: Vec::new(),
            mangle: mangle.into(),
        }
    }
}

impl ProgramAST {
    pub fn generate(&self) -> String {
        let mut result_code = String::new();
        let mut uid: usize = 0;

        let mut functions_codes = Vec::<String>::new();
        for (function_name, function_ast) in &self.functions {
            functions_codes.push(function_ast.generate(&self, &mut uid));
        }

        let main_call_statement =
            StatementASTNode::ExpressionAST(ExpressionASTNode::FunctionCallAST(FunctionCallAST {
                function_name: "main".into(),
                args: Vec::new(),
            }));
        main_call_statement.generate(&self, &mut Vec::new(), &mut result_code, &mut uid);

        for function_code in functions_codes {
            result_code.push_str(&function_code);
        }

        result_code
    }
}

impl FunctionAST {
    fn generate(&self, program_ast: &ProgramAST, uid: &mut usize) -> String {
        let mut result_code = String::new();
        let mut local_variables = Vec::<VariableScope>::new();
        let local_mangle = format!("_{}", uid);
        *uid += 1;
        local_variables.push(VariableScope::new(&local_mangle));
        for statement in &self.statements {
            statement.generate(program_ast, &mut local_variables, &mut result_code, uid);
        }
        result_code
    }
}

impl StatementASTNode {
    fn generate(
        &self,
        program_ast: &ProgramAST,
        local_variables: &mut Vec<VariableScope>,
        result_code: &mut String,
        uid: &mut usize,
    ) {
        match self {
            StatementASTNode::LocalVariableAST(lvs @ LocalVariableAST { name: lv }) => {
                let last_pos = local_variables.len() - 1;
                local_variables[last_pos].variables.push(lvs.clone());
            }
            StatementASTNode::AssignmentAST(AssignmentAST {
                target_var_name,
                value,
            }) => {
                //todo!();
                let target_var_name_mangled =
                    mangle_variable(target_var_name, &program_ast.variables, &local_variables)
                        .expect(&format!("Variable {} not defined", target_var_name));
                match value {
                    ExpressionASTNode::FunctionCallAST(fc) => {
                        fc.generate(program_ast, &target_var_name_mangled, result_code, uid);
                    }
                    ExpressionASTNode::StringLiteral(sl) => {
                        //TODO: escape string properly
                        result_code
                            .push_str(&format!("set {} \"{}\"", &target_var_name_mangled, sl));
                    }
                    ExpressionASTNode::NumberLiteral(nl) => {
                        result_code.push_str(&format!("set {} {}", &target_var_name_mangled, nl));
                    }
                    ExpressionASTNode::VariableReference(vr) => {
                        result_code.push_str(&format!("set {} {}", &target_var_name_mangled, vr));
                    }
                }
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
                let then_mangle = format!("_{}", uid);
                *uid += 1;
                let else_mangle = format!("_{}", uid);
                *uid += 1;

                let assign_condition_statement = StatementASTNode::AssignmentAST(AssignmentAST {
                    target_var_name: condition_buf.clone(),
                    value: condition.clone(),
                });
                assign_condition_statement.generate(program_ast, local_variables, result_code, uid);

                result_code.push_str(&format!("jump {} equal {} 0\n", else_label, condition_buf));
                local_variables.push(VariableScope::new(&then_mangle));
                for then_statement in then_block {
                    then_statement.generate(program_ast, local_variables, result_code, uid);
                }
                local_variables.pop();

                result_code.push_str(&else_label);
                result_code.push_str(":\n");
                local_variables.push(VariableScope::new(&else_mangle));
                for else_statement in else_block {
                    else_statement.generate(program_ast, local_variables, result_code, uid);
                }
                local_variables.pop();
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
                    do_statement.generate(program_ast, local_variables, result_code, uid);
                }

                result_code.push_str(&format!("jump {} always\n", while_begin_label));

                result_code.push_str(&while_end_label);
                result_code.push_str(":\n");
            }
        }
    }
}

impl FunctionCallAST {
    fn generate(
        &self,
        program_ast: &ProgramAST,
        target_variable: &str,
        result_code: &mut String,
        uid: &mut usize,
    ) {
        match self.function_name.as_str() {
            "add" => {
                assert!(
                    self.args.len() == 2,
                    "Builtin function add requires exactly two arguments"
                );

                let tmp_1 = format!("tmp_{}", uid);
                *uid += 1;
                let tmp_2 = format!("tmp_{}", uid);
                *uid += 1;

                //TODO

                result_code.push_str(&format!("op add {} {} {}", target_variable, tmp_1, tmp_2));
            }
            function_name => {
                let function_ast = program_ast
                    .functions
                    .get(function_name)
                    .expect(&format!("Function {} not defined", function_name));

                match &function_ast.style {
                    FunctionStyle::Normal => {
                        let ret_addr_buf = format!("ret_addr_{}", function_name);
                        let result_buf = format!("{}_result", function_name);

                        result_code.push_str(&format!(
                            "op add {} @counter 1\njump {} always\n",
                            ret_addr_buf, function_name
                        ));
                        result_code.push_str(&format!("set {} {}", target_variable, result_buf));
                    }
                    FunctionStyle::Inline => todo!(),
                }
            }
        }
    }
}
