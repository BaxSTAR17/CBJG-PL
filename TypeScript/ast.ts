import { analyze } from "./lexer";

export enum NodeRole {
    Program,
    Identifier,
    NumericLiteral,
    NullLiteral,
    BinaryExpression,
    BooleanLiteral,
    VarDeclaration,
    FunctionCall,
    StringLiteral,
    VarAssignment
}

export interface Statement {
    kind: NodeRole,
}

export interface Program extends Statement {
    kind: NodeRole.Program,
    body: Statement[]
}

export interface Expression extends Statement {}

export interface BinaryExpression extends Expression {
    kind: NodeRole.BinaryExpression,
    left: Expression,
    right: Expression,
    operator: string
}

export interface StringLiteral extends Expression {
    kind: NodeRole.StringLiteral,
    string: string
}

export interface Identifier extends Expression {
    kind: NodeRole.Identifier,
    string: string
}

export interface VarDeclaration extends Expression {
    kind: NodeRole.VarDeclaration,
    constant: boolean,
    identifier: string,
    value: Expression
}

export interface VarAssignment extends Expression {
    kind: NodeRole.VarAssignment,
    identifier: string,
    value: Expression
}

export interface NumericLiteral extends Expression {
    kind:NodeRole.NumericLiteral,
    number: number
}

export interface NullVoid extends Expression {
    kind: NodeRole.NullLiteral,
    value: "null"
}

export interface BooleanLiteral extends Expression {
    kind: NodeRole.BooleanLiteral,
    value: boolean
}

export interface FunctionCall extends Expression {
    kind: NodeRole.FunctionCall,
    fname: string,
    fparams: Expression[],
}


