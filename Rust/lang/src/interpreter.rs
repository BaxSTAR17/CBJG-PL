#[derive(Debug, Clone)]
pub enum RuntimeResult {
    NumericValue(NumericValue),
    StringValue(StringValue),
    NullValue(NullValue)
}

#[derive(Debug, Clone)]
pub struct NumericValue { pub value: i32 }

#[derive(Debug, Clone)]
pub struct StringValue { pub value: String }

#[derive(Debug, Clone)]
pub struct NullValue { pub value: String }

impl RuntimeResult {
    pub fn unwrap_to_i32(self) -> i32 {
        match self {
            RuntimeResult::NumericValue(x) => x.value,
            _ => panic!("Can't unwrap runtime result to numeric integer value")
        }
    }
}

pub fn evaluate(expression: crate::parser::Expression, env: &mut crate::environment::Environment) -> RuntimeResult {
    evaluate_parsed_expression(expression, env)
}

pub fn evaluate_parsed_expression(expression: crate::parser::Expression, env: &mut crate::environment::Environment) -> RuntimeResult {
    let exp = expression.clone();
    match expression {
        crate::parser::Expression::FunctionCall(x) => evaluate_function_call(exp, env),
        crate::parser::Expression::DeclareExp(x) => evaluate_declare_expression(exp, env),
        crate::parser::Expression::Node(x) => evaluate_node(x, env),
        crate::parser::Expression::BinaryExp(x) => evaluate_binary_expression(exp, env),
        _ => panic!("Can't parse expression")
    }
}

pub fn evaluate_function_call(expression: crate::parser::Expression, env: &mut crate::environment::Environment) -> RuntimeResult {
    let f = expression.unwrap_function_call();
    let mut params: Vec<RuntimeResult> = Vec::new();
    for param in f.fparams { params.push(evaluate_node(param, env)) }
    RuntimeResult::NumericValue(NumericValue { value: env.call_function(f.fname.unwrap_to_string(), params) })
}

pub fn evaluate_declare_expression(expression: crate::parser::Expression, env: &mut crate::environment::Environment) -> RuntimeResult {
    let exp = expression.clone();
    let result = evaluate_node(expression.unwrap_declare_exp().value, env);
    let res = result.clone();
    env.declare_variable(exp.unwrap_declare_exp().name.unwrap_to_string(), result);
    res
}

pub fn evaluate_binary_expression(expression: crate::parser::Expression, env: &mut crate::environment::Environment) -> RuntimeResult {
    let binary_expression: crate::parser::BinaryExpression = expression.unwrap_binary_exp();
    match binary_expression.operator {
        crate::parser::Node::PlusOperator(x) => RuntimeResult::NumericValue(NumericValue { value: evaluate_node(binary_expression.left, env).unwrap_to_i32() + evaluate_node(binary_expression.right, env).unwrap_to_i32() }),
        crate::parser::Node::MinusOperator(x) => RuntimeResult::NumericValue(NumericValue { value: evaluate_node(binary_expression.left, env).unwrap_to_i32() - evaluate_node(binary_expression.right, env).unwrap_to_i32() }),
        crate::parser::Node::MultiplyOperator(x) => RuntimeResult::NumericValue(NumericValue { value: evaluate_node(binary_expression.left, env).unwrap_to_i32() * evaluate_node(binary_expression.right, env).unwrap_to_i32() }),
        crate::parser::Node::DivOperator(x) => RuntimeResult::NumericValue(NumericValue { value: evaluate_node(binary_expression.left, env).unwrap_to_i32() / evaluate_node(binary_expression.right, env).unwrap_to_i32() }),
        crate::parser::Node::ModOperator(x) => RuntimeResult::NumericValue(NumericValue { value: evaluate_node(binary_expression.left, env).unwrap_to_i32() % evaluate_node(binary_expression.right, env).unwrap_to_i32() }),
        _ => panic!("Can't interpret binary expression with operator '{}'", binary_expression.operator.unwrap_to_string())
    }
}

pub fn evaluate_expression(expression: crate::parser::Node, env: &mut crate::environment::Environment) -> RuntimeResult {
    evaluate(crate::parser::Parser::create_ast(&mut crate::lexer::Lexer::collect_dabs(expression.unwrap_to_string().as_str())).current_expression(), env)
}

pub fn evaluate_identifier(name: String, env: &mut crate::environment::Environment) -> RuntimeResult {
    env.call_variable(name)
}

pub fn evaluate_node(node: crate::parser::Node, env: &mut crate::environment::Environment) -> RuntimeResult {
    let nd = node.clone();
    match node {
        crate::parser::Node::NumberLiteral(x) => RuntimeResult::NumericValue(NumericValue { value: x.parse::<i32>().unwrap() }),
        crate::parser::Node::Expression(x) => evaluate_expression(nd, env),
        crate::parser::Node::Identifier(x) => evaluate_identifier(x, env),
        crate::parser::Node::StringLiteral(x) => RuntimeResult::StringValue(StringValue { value: nd.unwrap_to_string()}),
        _ => panic!("Can't parse node: {}", nd.unwrap_to_string())
    }
}
