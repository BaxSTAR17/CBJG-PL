#[derive(Clone, Debug)]
pub struct Parser {
    pub ast: Vec<Expression>
}

#[derive(Clone, Debug)]
pub enum Node {
    PlusOperator(String),
    MinusOperator(String),
    MultiplyOperator(String),
    DivOperator(String),
    ModOperator(String),
    StringLiteral(String),
    Identifier(String),
    NumberLiteral(String),
    // Int16Number(String),
    // Int32Number(String),
    // Int64Number(String),
    Expression(String),
    // Accessibility(crate::lexer::Material),
    Typing(crate::lexer::Material)
}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    pub left: Node,
    // pub leftbp: u8,
    pub operator: Node,
    // pub rightbp: u8,
    pub right: Node,
}

#[derive(Clone, Debug)]
pub struct DeclareExpression {
    pub typing: Node,
    pub access: Option<Node>,
    pub name: Node,
    pub value: Node
}

#[derive(Clone, Debug)]
pub struct AssignExpression {
    pub name: Node,
    pub value: Node
}

#[derive(Clone, Debug)]
pub struct FunctionCall {
    pub fname: Node,
    pub fparams: Vec<Node>
}

#[derive(Clone, Debug)]
pub enum Expression {
    BinaryExp(BinaryExpression),
    Node(Node),
    DeclareExp(DeclareExpression),
    // AssignExp(AssignExpression),
    FunctionCall(FunctionCall)
}

impl Node {
    pub fn unwrap_to_string(self) -> String {
        match self {
            Node::NumberLiteral(x) | Node::PlusOperator(x) | Node::Expression(x) |
            Node::MinusOperator(x) | Node::MultiplyOperator(x) | Node::ModOperator(x) |
            Node::Identifier(x) | Node::StringLiteral(x) => x,
            _ => panic!("Can't unwrap node: to string")
        }
    }
}

impl Expression {
    pub fn unwrap_to_node(self) -> Node {
        let debugvec = vec![self.clone()];
        match self {
            Expression::Node(x) => x,
            Expression::BinaryExp(x) => Node::Expression(x.left.unwrap_to_string() + &x.operator.unwrap_to_string() + &x.right.unwrap_to_string()),
            _ => panic!("Can't unwrap expression to node: {:?}", debugvec)
        }
    }

    pub fn unwrap_binary_exp(self) -> BinaryExpression {
        match self {
            Expression::BinaryExp(x) => x,
            _ => panic!("Can't unwrap expression to binary expression")
        }
    }

    pub fn unwrap_declare_exp(self) -> DeclareExpression {
        match self {
            Expression::DeclareExp(x) => x,
            _ => panic!("Can't unwrap expression to a declare expression")
        }
    }

    pub fn unwrap_function_call(self) -> FunctionCall {
        match self {
            Expression::FunctionCall(x) => x,
            _ => panic!("Can't unwrap expression to a declare expression")
        }
    }
}

impl Parser {
    pub fn create_ast(lexer: &mut crate::lexer::Lexer) -> Parser {
        let mut ast: Vec<Expression> = Vec::new();
        while lexer.clone().current_dab().material != crate::lexer::Material::EndFile { ast.push(Parser::parse_dab(lexer)); }
        ast.reverse();
        Parser { ast }
    }

    pub fn parse_dab(lexer: &mut crate::lexer::Lexer) -> Expression { 
        match lexer.clone().current_dab().material {
            crate::lexer::Material::AutoTypeMat => Parser::parse_declare_exp(lexer),
            _ => Parser::parse_additive_exp(lexer)
        }
    }

    pub fn parse_declare_exp(lexer: &mut crate::lexer::Lexer) -> Expression {
        let typing = Node::Typing(lexer.mov_dab().material);
        let identifier = Node::Identifier(lexer.mov_dab().value);
        if lexer.mov_dab().material != crate::lexer::Material::EqualsMat { panic!("Expected assignment after variable declaration!") }
        let value = Parser::parse_dab(lexer).unwrap_to_node();
        Expression::DeclareExp(DeclareExpression { typing, access: Option::None, name: identifier, value })
    }

    pub fn parse_additive_exp(lexer: &mut crate::lexer::Lexer) -> Expression {
        if lexer.peek_dab().material == crate::lexer::Material::PlusOperator || lexer.peek_dab().material == crate::lexer::Material::MinusOperator {
            let left = Parser::parse_multiplicative_exp(lexer).unwrap_to_node();
            let operator = match lexer.clone().current_dab().material.clone() {
                crate::lexer::Material::PlusOperator => Node::PlusOperator(lexer.clone().mov_dab().value),
                crate::lexer::Material::MinusOperator => Node::MinusOperator(lexer.clone().mov_dab().value),
                _ => panic!("Can't parse binary expression with operator '{}'", lexer.clone().mov_dab().value)
            };
            lexer.mov_dab();
            let right = Parser::parse_multiplicative_exp(lexer).unwrap_to_node();
            Expression::BinaryExp(BinaryExpression { left, operator, right })
        }
        else { Parser::parse_multiplicative_exp(lexer) }
    }

    pub fn parse_multiplicative_exp(lexer: &mut crate::lexer::Lexer) -> Expression {
        if lexer.peek_dab().material == crate::lexer::Material::MultOperator || lexer.peek_dab().material == crate::lexer::Material::DivOperator || lexer.peek_dab().material == crate::lexer::Material::ModOperator {
            let left = Parser::parse_function_call(lexer).unwrap_to_node();
            let operator = match lexer.clone().current_dab().material.clone() {
                crate::lexer::Material::MultOperator => Node::MultiplyOperator(lexer.clone().mov_dab().value),
                crate::lexer::Material::DivOperator => Node::DivOperator(lexer.clone().mov_dab().value),
                crate::lexer::Material::ModOperator => Node::ModOperator(lexer.clone().mov_dab().value),
                _ => panic!("Can't parse binary expression with operator '{}'", lexer.clone().mov_dab().value)
            };
            lexer.mov_dab();
            let right = Parser::parse_function_call(lexer).unwrap_to_node();
            Expression::BinaryExp(BinaryExpression { left, operator, right })
        }
        else { Parser::parse_function_call(lexer) }
    }

    pub fn parse_function_call(lexer: &mut crate::lexer::Lexer) -> Expression {
        if lexer.clone().current_dab().material == crate::lexer::Material::IdentifierMat && lexer.peek_dab().material == crate::lexer::Material::OpenParent {
            let fname = Node::Identifier(lexer.mov_dab().value);
            Parser::expect_dab(lexer, crate::lexer::Material::OpenParent);
            let mut fparams: Vec<Node> = Vec::new();
            'params: while lexer.clone().current_dab().material != crate::lexer::Material::CloseParent { 
                fparams.push(Parser::parse_dab(lexer).unwrap_to_node());
                if lexer.clone().current_dab().material != crate::lexer::Material::CommaMat { break 'params; }
                else { lexer.mov_dab(); continue; }
            }
            lexer.mov_dab();
            Expression::FunctionCall(FunctionCall { fname, fparams })
        }
        else { Parser::parse_primary_exp(lexer) }
    }

    pub fn parse_primary_exp(lexer: &mut crate::lexer::Lexer) -> Expression {
        match lexer.clone().current_dab().material {
            crate::lexer::Material::IdentifierMat => Expression::Node(Node::Identifier(lexer.mov_dab().value)),
            crate::lexer::Material::NumberMat => Expression::Node(Node::NumberLiteral(lexer.mov_dab().value)),
            crate::lexer::Material::StringMat => Expression::Node(Node::StringLiteral(lexer.mov_dab().value)),
            _ => panic!("Unexpected word was found '{}', -> {}", lexer.clone().current_dab().value, lexer.clone().peek_dab().value)
        }
    }

    pub fn current_expression(self) -> Expression { self.ast.last().unwrap().clone() }
    pub fn mov_expression(&mut self) -> Expression { self.ast.pop().unwrap() }
    pub fn expect_dab(lexer: &mut crate::lexer::Lexer, material: crate::lexer::Material) -> crate::lexer::Dabloon { 
        if lexer.clone().current_dab().material == material { lexer.mov_dab() }
        else { panic!("Unexpected word in the expression: '{}'", lexer.clone().current_dab().value) }
    }
}

// fn infix_binding(operator: crate::lexer::Dabloon) -> (u8, u8) {
//     match operator.material {
//         crate::lexer::Material::PlusOperator | crate::lexer::Material::MinusOperator => (3, 3),
//         crate::lexer::Material::MultOperator | crate::lexer::Material::DivOperator | crate::lexer::Material::ModOperator => (5, 5),
//         _ => panic!("Unexpected operator was found '{}'", operator.value)
//     }
// }