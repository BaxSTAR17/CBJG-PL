import { readFileSync } from "fs";

export enum Role {
    Identifier,
    Equals,
    OpenParent,
    CloseParent,
    OpenBrace,
    CloseBrace,
    OpenBrack,
    CloseBrack,
    Number,
    PlusOperator,
    MinusOperator,
    MultOperator,
    DivOperator,
    EndLine,
    TypingInteger,
    TypingBoolean,
    TypingString,
    TypingFloat,
    FunctionDeclaration,
    ForLoopDeclaration,
    WhileLoopDeclaration,
    DoLoopDeclaration,
    String,
    ColonOperator,
    ModOperator,
    Null,
    Comma,
    BoolT,
    BoolF,
    LetDeclare,
    ConstDeclare,
    EndFile,
    IfCond,
    ElsifCond,
    ElseCond,
    SwitchCond,
    CaseCond,
}

export const KeyWords: Record<string, Role> = {
    "Ghen": Role.PlusOperator,
    "gHen": Role.MinusOperator,
    "ghEn": Role.MultOperator,
    "gheN": Role.DivOperator,
    "GHEN": Role.ModOperator,
    "joshua": Role.FunctionDeclaration,
    "bax": Role.WhileLoopDeclaration,
    "ter": Role.DoLoopDeclaration,
    "baxter": Role.ForLoopDeclaration,
    "carloss": Role.Null,
    "ghen": Role.Equals,
    "Zer": Role.BoolT,
    "Xeus": Role.BoolF,
    "car": Role.LetDeclare,
    "LOS": Role.ConstDeclare,
    "BAX": Role.IfCond,
    "BAXTER" : Role.ElsifCond,
    "TER" : Role.ElseCond,
    "BAXSTAR" : Role.SwitchCond,
    "star" : Role.CaseCond
}

function alphaCheck(value: string): boolean  {
    return value.toUpperCase() != value.toLowerCase();
}

export interface Word {
    value: string,
    role: Role
}

export function makeWord(value = "", role: Role): Word { return { value, role }; }

export function analyze(codes: string): Word[] {
    const words = new Array<Word>();
    const src = codes.split('');
    while (src.length > 0) {
        if(src[0] == '(') words.push(makeWord(src.shift(), Role.OpenParent));
        else if(src[0] == ')') words.push(makeWord(src.shift(), Role.CloseParent));
        else if(src[0] == '[') words.push(makeWord(src.shift(), Role.OpenBrack));
        else if(src[0] == ']') words.push(makeWord(src.shift(), Role.CloseBrack));
        else if(src[0] == '{') words.push(makeWord(src.shift(), Role.OpenBrace));
        else if(src[0] == '}') words.push(makeWord(src.shift(), Role.CloseBrace));
        else if(src[0] == '=') words.push(makeWord(src.shift(), Role.Equals));
        else if(src[0] == ',') words.push(makeWord(src.shift(), Role.Comma));
        else if(src[0] == ';') words.push(makeWord(src.shift(), Role.EndLine));
        else {
            //for multicharacter words
            if(!(isNaN(parseInt(src[0])))) {
                let numval = "";
                while(src.length > 0 && !(isNaN(parseInt(src[0])))) numval += src.shift();
                words.push(makeWord(numval, Role.Number));
            }
            else if(src[0] == '"') {
                src.shift();
                let string = "";
                while(src.length > 0 && src[0] != '"') string += src.shift();
                words.push(makeWord(string, Role.String));
                src.shift();
            } 
            else if(alphaCheck(src[0])) {
                let id = "";
                while(src.length > 0 && alphaCheck(src[0])) id += src.shift();
                words.push(makeWord(id, KeyWords[id] == undefined ? Role.Identifier : KeyWords[id]));
            }
            else if(src[0] == ' ' || src[0] == '\n' || src[0] == '\t' || src[0] == '\r') src.shift();
            else {
                throw `Unknown character/word was found: ${src}`;
                process.exit();
            }
        }
    }
    words.push(makeWord("EOF", Role.EndFile));
    return words;
}

// const code = readFileSync("./hello.cbjg", 'utf-8');
// for(const word of analyze(code)) console.log(word);
