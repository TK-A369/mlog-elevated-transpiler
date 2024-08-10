use crate::parser::*;

use lazy_static;

#[derive(Debug)]
struct VariableScope {
    variables: Vec<LocalVariableAST>,
    mangle: String,
}

fn mangle_variable(
    variable_name: &str,
    global_variables: &std::collections::BTreeMap<String, GlobalVariableAST>,
    local_variables: &[VariableScope],
) -> Option<String> {
    if variable_name.chars().next().unwrap_or('@') == '@' {
        return Some(variable_name.into());
    }

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
        println!("Not found variable {}", variable_name);
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

        let mut functions_codes = Vec::<(&str, String)>::new();
        for (function_name, function_ast) in &self.functions {
            functions_codes.push((&function_name, function_ast.generate(&self, &mut uid)));
        }

        let main_call_statement =
            StatementASTNode::ExpressionAST(ExpressionASTNode::FunctionCallAST(FunctionCallAST {
                function_name: "main".into(),
                args: Vec::new(),
            }));
        main_call_statement.generate(&self, &mut Vec::new(), &mut result_code, &mut uid);

        for function_code in functions_codes {
            result_code.push_str(&function_code.0);
            result_code.push_str(":\n");
            result_code.push_str(&function_code.1);
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
                match value {
                    ExpressionASTNode::FunctionCallAST(fc) => {
                        fc.generate(
                            program_ast,
                            local_variables,
                            target_var_name,
                            result_code,
                            uid,
                        );
                    }
                    ExpressionASTNode::StringLiteral(sl) => {
                        //TODO: escape string properly
                        result_code.push_str(&format!(
                            "set {} \"{}\"\n",
                            mangle_variable(
                                target_var_name,
                                &program_ast.variables,
                                local_variables
                            )
                            .unwrap(),
                            sl
                        ));
                    }
                    ExpressionASTNode::NumberLiteral(nl) => {
                        result_code.push_str(&format!(
                            "set {} {}\n",
                            mangle_variable(
                                target_var_name,
                                &program_ast.variables,
                                local_variables
                            )
                            .unwrap(),
                            nl
                        ));
                    }
                    ExpressionASTNode::VariableReference(vr) => {
                        result_code.push_str(&format!(
                            "set {} {}\n",
                            mangle_variable(
                                target_var_name,
                                &program_ast.variables,
                                local_variables
                            )
                            .unwrap(),
                            mangle_variable(vr, &program_ast.variables, local_variables).unwrap()
                        ));
                    }
                }
            }
            StatementASTNode::ExpressionAST(expr) => match expr {
                ExpressionASTNode::FunctionCallAST(fc) => {
                    let fc_mangle = format!("_{}", uid);
                    *uid += 1;

                    local_variables.push(VariableScope::new(&fc_mangle));
                    let blackhole_declaration_statement =
                        StatementASTNode::LocalVariableAST(LocalVariableAST {
                            name: "blackhole".into(),
                        });
                    blackhole_declaration_statement.generate(
                        program_ast,
                        local_variables,
                        result_code,
                        uid,
                    );

                    fc.generate(program_ast, local_variables, "blackhole", result_code, uid);

                    local_variables.pop();
                }
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
                let cond_var = format!("cond_{}", uid);
                *uid += 1;
                let cond_mangle = format!("_{}", uid);
                *uid += 1;
                let then_mangle = format!("_{}", uid);
                *uid += 1;
                let else_mangle = format!("_{}", uid);
                *uid += 1;

                local_variables.push(VariableScope::new(&cond_mangle));
                let cond_var_statement = StatementASTNode::LocalVariableAST(LocalVariableAST {
                    name: cond_var.clone(),
                });
                cond_var_statement.generate(program_ast, local_variables, result_code, uid);
                let assign_condition_statement = StatementASTNode::AssignmentAST(AssignmentAST {
                    target_var_name: cond_var.clone(),
                    value: condition.clone(),
                });
                assign_condition_statement.generate(program_ast, local_variables, result_code, uid);
                result_code.push_str(&format!(
                    "jump {} equal {} 0\n",
                    else_label,
                    mangle_variable(&cond_var, &program_ast.variables, local_variables).unwrap()
                ));
                local_variables.pop();

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
                let while_mangle = format!("_{}", uid);
                *uid += 1;

                local_variables.push(VariableScope::new(&while_mangle));
                let declare_cond_buf_statement =
                    StatementASTNode::LocalVariableAST(LocalVariableAST {
                        name: condition_buf.clone(),
                    });
                declare_cond_buf_statement.generate(program_ast, local_variables, result_code, uid);

                result_code.push_str(&while_begin_label);
                result_code.push_str(":\n");

                let assign_condition_statement = StatementASTNode::AssignmentAST(AssignmentAST {
                    target_var_name: condition_buf.clone(),
                    value: condition.clone(),
                });
                assign_condition_statement.generate(program_ast, local_variables, result_code, uid);

                result_code.push_str(&format!(
                    "jump {} equal {} 0\n",
                    while_end_label,
                    mangle_variable(&condition_buf, &program_ast.variables, local_variables)
                        .unwrap()
                ));

                for do_statement in do_block {
                    do_statement.generate(program_ast, local_variables, result_code, uid);
                }

                result_code.push_str(&format!("jump {} always\n", while_begin_label));

                result_code.push_str(&while_end_label);
                result_code.push_str(":\n");

                local_variables.pop();
            }
        }
    }
}

fn make_tmp_variable(
    value: &Option<ExpressionASTNode>,
    program_ast: &ProgramAST,
    local_variables: &mut Vec<VariableScope>,
    result_code: &mut String,
    uid: &mut usize,
) -> String {
    let tmp_name = format!("tmp_{}", uid);
    *uid += 1;

    let declare_tmp_statement = StatementASTNode::LocalVariableAST(LocalVariableAST {
        name: tmp_name.clone(),
    });
    declare_tmp_statement.generate(program_ast, local_variables, result_code, uid);
    println!(
        "Declaring temporary variable {}, after mangling {}",
        tmp_name,
        mangle_variable(&tmp_name, &program_ast.variables, local_variables).unwrap()
    );
    println!("Backtrace: {}", std::backtrace::Backtrace::capture());

    if let Some(value_expr) = value {
        println!("And assigning value {:?} to it", &value_expr);
        let assignment_statement = StatementASTNode::AssignmentAST(AssignmentAST {
            target_var_name: tmp_name.clone(),
            value: value_expr.clone(),
        });
        assignment_statement.generate(program_ast, local_variables, result_code, uid);
    }

    tmp_name
}

fn make_tmp_variables<const COUNT: usize>(
    values: &[Option<ExpressionASTNode>; COUNT],
    program_ast: &ProgramAST,
    local_variables: &mut Vec<VariableScope>,
    result_code: &mut String,
    uid: &mut usize,
) -> [String; COUNT] {
    const EMPTY_STRING: String = String::new();
    let mut result: [String; COUNT] = [EMPTY_STRING; COUNT];
    for i in 0..COUNT {
        result[i] = make_tmp_variable(&values[i], program_ast, local_variables, result_code, uid);
    }
    result
}

lazy_static::lazy_static! (
    static ref BINARY_OPS: Vec<&'static str> = {
        vec!["add", "sub", "mul", "div", "equal"] //TODO: Add rest
    };

    static ref BUILTIN_FUNCTIONS: std::sync::Mutex<
        std::collections::BTreeMap<&'static str, Box<dyn FnMut(
            &[ExpressionASTNode], &ProgramAST, &mut Vec<VariableScope>, &str, &mut String, &mut usize
        ) + Send + Sync>>
    > = {
        let mut m: std::collections::BTreeMap<&'static str, Box<dyn FnMut(
            &[ExpressionASTNode], &ProgramAST, &mut Vec<VariableScope>, &str, &mut String, &mut usize
        ) + Send + Sync>> = std::collections::BTreeMap::new();

        for binary_op in BINARY_OPS.iter() {
            m.insert(binary_op, Box::new(
                move |
                    args: &[ExpressionASTNode],
                    program_ast: &ProgramAST,
                    local_variables: &mut Vec<VariableScope>,
                    target_variable: &str,
                    result_code: &mut String,
                    uid: &mut usize
                | {
                    println!("Binary operation {} called with arguments {:?}", binary_op, args);
                    let tmps: [String; 2] = make_tmp_variables(
                        &[Some(args[0].clone()), Some(args[1].clone())],
                        program_ast, local_variables, result_code, uid
                    );
                    println!("Tmps: {:?}", tmps);

                    result_code.push_str(&format!(
                        "op {} {} {} {}\n",
                        binary_op,
                        mangle_variable(target_variable, &program_ast.variables, local_variables)
                            .unwrap(),
                        mangle_variable(&tmps[0], &program_ast.variables, local_variables).unwrap(),
                        mangle_variable(&tmps[1], &program_ast.variables, local_variables).unwrap()
                    ));
                }
            ));
        }

        m.insert("radar", Box::new(
            |
                args: &[ExpressionASTNode],
                program_ast: &ProgramAST,
                local_variables: &mut Vec<VariableScope>,
                target_variable: &str,
                result_code: &mut String,
                uid: &mut usize
            | {
                result_code.push_str(&format!(
                    "radar {} {} {} {} {} {} {}\n",
                    //1st filter
                    if let ExpressionASTNode::StringLiteral(arg) = &args[0] {
                        arg
                    } else {
                        panic!("1st argument to radar function must be string")
                    },
                    //2nd filter
                    if let ExpressionASTNode::StringLiteral(arg) = &args[1] {
                        arg
                    } else {
                        panic!("2nd argument to radar function must be string")
                    },
                    //3rd filter
                    if let ExpressionASTNode::StringLiteral(arg) = &args[2] {
                        arg
                    } else {
                        panic!("3rd argument to radar function must be string")
                    },
                    //sort criterion
                    if let ExpressionASTNode::StringLiteral(arg) = &args[3] {
                        arg
                    } else {
                        panic!("4th argument to radar function must be string")
                    },
                    //object which will be used for detection
                    mangle_variable(
                        if let ExpressionASTNode::VariableReference(arg) = &args[4] {
                            &arg
                        } else {
                            panic!("5th argument to radar function must be variable reference")
                        }, &program_ast.variables, local_variables).unwrap(),
                    //order
                    if let ExpressionASTNode::NumberLiteral(arg) = args[5] {
                        arg
                    } else {
                        panic!("6th argument to radar function must be number")
                    },
                    //output variable
                    mangle_variable(target_variable, &program_ast.variables, local_variables).unwrap()
                ));
            }
        ));

        m.insert("ubind", Box::new(
            |
                args: &[ExpressionASTNode],
                program_ast: &ProgramAST,
                local_variables: &mut Vec<VariableScope>,
                target_variable: &str,
                result_code: &mut String,
                uid: &mut usize
            | {
                result_code.push_str(&format!(
                    "ubind {}\n",
                    mangle_variable(
                        if let ExpressionASTNode::VariableReference(arg) = &args[0] {
                            &arg
                        } else {
                            panic!("1st argument to ubind function must be variable reference");
                        }, &program_ast.variables, local_variables).unwrap()
                ));
            }
        ));

        m.insert("ucontrolMove", Box::new(
            |
                args: &[ExpressionASTNode],
                program_ast: &ProgramAST,
                local_variables: &mut Vec<VariableScope>,
                target_variable: &str,
                result_code: &mut String,
                uid: &mut usize
            | {
                let tmps: [String; 2] = make_tmp_variables(
                    &[Some(args[0].clone()), Some(args[1].clone())],
                    program_ast, local_variables, result_code, uid
                );

                result_code.push_str(&format!(
                    "ucontrol move {} {} 0 0 0\n",
                    mangle_variable(&tmps[0], &program_ast.variables, local_variables).unwrap(),
                    mangle_variable(&tmps[1], &program_ast.variables, local_variables).unwrap()
                ));
            }
        ));

        m.insert("ucontrolWithin", Box::new(
            |
                args: &[ExpressionASTNode],
                program_ast: &ProgramAST,
                local_variables: &mut Vec<VariableScope>,
                target_variable: &str,
                result_code: &mut String,
                uid: &mut usize
            | {
                let tmps: [String; 3] = make_tmp_variables(
                    &[Some(args[0].clone()), Some(args[1].clone()), Some(args[2].clone())],
                    program_ast, local_variables, result_code, uid
                );

                result_code.push_str(&format!(
                    "ucontrol move {} {} {} {} 0\n",
                    mangle_variable(&tmps[0], &program_ast.variables, local_variables).unwrap(),
                    mangle_variable(&tmps[1], &program_ast.variables, local_variables).unwrap(),
                    mangle_variable(&tmps[2], &program_ast.variables, local_variables).unwrap(),
                    mangle_variable(target_variable, &program_ast.variables, local_variables).unwrap()
                ));
            }
        ));

        std::sync::Mutex::new(m)
    };
);

impl FunctionCallAST {
    fn generate(
        &self,
        program_ast: &ProgramAST,
        local_variables: &mut Vec<VariableScope>,
        target_variable: &str,
        result_code: &mut String,
        uid: &mut usize,
    ) {
        println!(
            "Calling function {} with arguments {:?} and saving result to variable {}",
            &self.function_name, self.args, target_variable
        );
        println!("Local variables:\n{:?}", local_variables);
        let mut builtin_functions_locked = BUILTIN_FUNCTIONS.lock().unwrap();
        match self.function_name.as_str() {
            builtin_fn if builtin_functions_locked.get(builtin_fn).is_some() => {
                let local_mangle = format!("_{}", uid);
                *uid += 1;
                local_variables.push(VariableScope::new(&local_mangle));

                let builtin_fn_generator_fn = builtin_functions_locked.get_mut(builtin_fn).unwrap();
                println!("Calling builtin function {}", builtin_fn);
                (*builtin_fn_generator_fn)(
                    &self.args,
                    program_ast,
                    local_variables,
                    target_variable,
                    result_code,
                    uid,
                );

                local_variables.pop();
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
                        result_code.push_str(&format!("set {} {}\n", target_variable, result_buf));
                    }
                    FunctionStyle::Inline => todo!(),
                }
            }
        }
    }
}
