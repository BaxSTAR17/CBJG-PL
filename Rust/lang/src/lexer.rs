#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Material {
    Int16Mat,
    Int32Mat,
    Int64Mat,
    EqualsMat,
    OpenParent,
    CloseParent,
    EndLine,
    EndFile,
    NumberMat,
    StringMat,
    StringTypeMat,
    PrivateConstMat,
    PrivateMutMat,
    PublicConstMat,
    PublicMutMat,
    IdentifierMat,
    PlusOperator,
    MinusOperator,
    MultOperator,
    DivOperator,
    ModOperator,
    AutoTypeMat,
    ConstantMat,
    PublicMat,
    CommaMat,
    PrivateMat
}

#[derive(Clone, Debug)]
pub struct Dabloon {
    pub material: Material,
    pub value: String
}

#[derive(Clone, Debug)]
pub struct Lexer {
    pub dabloons: Vec<Dabloon>
}

impl Lexer {
    pub fn collect_dabs(src: &str) -> Lexer {
        let mut keywords: std::collections::HashMap<String, Material> = std::collections::HashMap::new();
        keywords.insert("i2".to_string(), Material::Int16Mat);
        keywords.insert("i4".to_string(), Material::Int32Mat);
        keywords.insert("i8".to_string(), Material::Int64Mat);
        keywords.insert("i2".to_string(), Material::Int16Mat);
        keywords.insert("str".to_string(), Material::StringTypeMat);
        keywords.insert("car".to_string(), Material::AutoTypeMat);
        keywords.insert("Ghen".to_string(), Material::PlusOperator);
        keywords.insert("gHen".to_string(), Material::MinusOperator);
        keywords.insert("ghEn".to_string(), Material::MultOperator);
        keywords.insert("gheN".to_string(), Material::DivOperator);
        keywords.insert("GHEN".to_string(), Material::ModOperator);
        let code: Vec<char> = src.chars().collect();
        let mut i: usize = 0;
        let mut dabloons: Vec<Dabloon> = Vec::new();
        while i < code.len() as usize {
            if code[i] == '(' { dabloons.push(Dabloon { material: Material::OpenParent, value: code[i].to_string()}) }
            else if code[i] == ')' { dabloons.push(Dabloon { material: Material::CloseParent, value: code[i].to_string()}) }
            else if code[i] == '=' { dabloons.push(Dabloon { material: Material::EqualsMat, value: code[i].to_string()}) }
            else if code[i] == ',' { dabloons.push(Dabloon { material: Material::CommaMat, value: code[i].to_string()}) }
            else {
                let mut dabloon: String = "".to_string();
                if code[i] == '"' {
                    i += 1;
                    while i < code.len() as usize && code[i] != '"'  {
                        // if src[i+1] ==  && code[i] != '"' {panic!("Expected closing quote, string is leaking")};
                        dabloon.push(code[i]);
                        i += 1;
                    }
                    dabloons.push(Dabloon {material: Material::StringMat, value: dabloon});   
                }
                else if i+1 < code.len() &&code[i] == '<' && code[i+1] == '>' {
                    i += 2;
                    dabloon = "<>".to_string();
                    dabloons.push(Dabloon {material: Material::PrivateConstMat, value: dabloon});
                }
                else if  i+1 < code.len() && code[i] == '-' && code[i+1] == '>' {
                    i += 2;
                    dabloon = "->".to_string();
                    dabloons.push(Dabloon {material: Material::PublicConstMat, value: dabloon});
                }
                else if i+2 < code.len() && code[i] == '<' && code[i+1] == '-' && code[i+2] == '>' {
                    i += 3;
                    dabloon = "<->".to_string();
                    dabloons.push(Dabloon {material: Material::PublicMutMat, value: dabloon});
                }
                else if  i+1 < code.len() && code[i] == '<' && code[i+1] == '-' {
                    i += 2;
                    dabloon = "<-".to_string();
                    dabloons.push(Dabloon {material: Material::PrivateMutMat, value: dabloon});
                }
                else if code[i].is_numeric() {
                    dabloon.push(code[i]);
                    while i+1 < code.len() && code[i+1].is_numeric() {
                        i += 1;
                        dabloon.push(code[i]);
                    }
                    dabloons.push(Dabloon {material: Material::NumberMat, value: dabloon});
                }
                else if code[i].is_alphabetic() {
                    dabloon.push(code[i]);
                    'identify: while i+1 < code.len() && code[i+1].is_alphanumeric() && code[i+1] != ' ' && code[i+1] != '\n' && code[i+1] != '\t' && code[i+1] != '\r' {
                        i += 1;
                        dabloon.push(code[i]);
                        if keywords.contains_key(&dabloon) { break 'identify; }
                    }
                    let dabloonmat: Material = if keywords.contains_key(&dabloon) {keywords.get(&dabloon).unwrap().clone()} else {Material::IdentifierMat};
                    dabloons.push(Dabloon {material: dabloonmat, value: dabloon});
                }
                else if code[i] == ' ' || code[i] == '\n' || code[i] == '\t' || code[i] == '\r' { i += 1; continue; }
                else { }
            }
            i += 1;
        }
        let eof: String = "EndOfFile".to_string();
        dabloons.push(Dabloon { material: Material::EndFile, value: eof});
        dabloons.reverse();
        Lexer { dabloons }
    }

    pub fn current_dab(self) -> Dabloon { self.dabloons.last().unwrap().clone() }
    pub fn mov_dab(&mut self) -> Dabloon { self.dabloons.pop().unwrap() }
    pub fn peek_dab(&mut self) -> Dabloon { self.dabloons[self.dabloons.len() - 2].clone() }
}