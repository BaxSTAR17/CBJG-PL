import { NodeRole, Program, Statement, Expression, BinaryExpression, Identifier, NumericLiteral, NullVoid, VarDeclaration, VarAssignment, StringLiteral, FunctionCall } from "./ast.ts";
import { Role, KeyWords } from "./lexer.ts";
import Ecosystem from './environment.ts';

export type ValueRole = "null"
| "number"
| "bool"
| "identifier"
| "string";

export interface RuntimeResult {
    type: ValueRole,
}

export interface NullResult extends RuntimeResult {
    type: "null",
    value: "null"
}

export interface NumberResult extends RuntimeResult {
    type: "number",
    value: number
}

export interface StringResult extends RuntimeResult {
    type: "string",
    value: string
}

export interface BooleanResult extends RuntimeResult {
    type: "bool",
    value: boolean
}

export function evaluateProgram(program: Program, eco: Ecosystem): RuntimeResult {
    let currentresult: RuntimeResult = { type: "null", value: "null"} as NullResult;
    for(const statement of program.body) currentresult = evaluateStatement(statement, eco);
    return currentresult;
}

export function evaluateBinaryExp(statement: BinaryExpression, eco: Ecosystem): RuntimeResult {
    const left = evaluateStatement(statement.left, eco) as NumberResult;
    const right = evaluateStatement(statement.right, eco) as NumberResult;
    let result: number = 0;
    if(left.type == "number" && right.type == "number") {
        switch(KeyWords[statement.operator]) {
            case Role.PlusOperator: 
                result = left.value + right.value;
                break;
            case Role.MinusOperator: 
                result = left.value - right.value;
                break;
            case Role.MultOperator:
                result = left.value * right.value;
                break;
            case Role.DivOperator:
                result = left.value / right.value;
                break;
            case Role.ModOperator:
                result = left.value % right.value;
                break;
        }
        return { type: "number", value: result } as NumberResult;
    }
    return { type: "null", value: "null"} as NullResult;
}

export function evaluateVarDeclaration(statement: VarDeclaration, eco: Ecosystem): RuntimeResult {
    let value: RuntimeResult = evaluateStatement(statement.value, eco);
    switch(statement.constant) {
        case true:
            return eco.declareConst(statement.identifier, value);
        case false:
            return eco.declareVar(statement.identifier, value);
    }
}

export function evaluateVarAssignment(statement: VarAssignment, eco: Ecosystem): RuntimeResult {
    let value: RuntimeResult = evaluateStatement(statement.value, eco);
    return eco.assignVar(statement.identifier, value);
}

export function evaluateFunctionCall(statement: FunctionCall, eco: Ecosystem): RuntimeResult {
    let args: RuntimeResult[] = [];
    while(statement.fparams.length > 0) args.push(evaluateStatement(statement.fparams.shift() as Statement, eco));
    return eco.callFunc(statement.fname, args);
}

export function evaluateStatement(statement: Statement, eco: Ecosystem): RuntimeResult {
    switch(statement.kind) {
        case NodeRole.StringLiteral:
            return {
                type: "string",
                value: (statement as StringLiteral).string
            } as StringResult;
        case NodeRole.Identifier:
            const stmt: Identifier = statement as Identifier;
            return eco.callVar(stmt.string);
        case NodeRole.NumericLiteral:
            return {
                type: "number",
                value: (statement as NumericLiteral).number
            } as NumberResult;
        case NodeRole.FunctionCall:
            return evaluateFunctionCall(statement as FunctionCall, eco);
        case NodeRole.NullLiteral:
            return {
                type: "null",
                value: "null"
            } as NullResult;
        case NodeRole.BinaryExpression:
            return evaluateBinaryExp(statement as BinaryExpression, eco);
        case NodeRole.VarDeclaration:
            return evaluateVarDeclaration(statement as VarDeclaration, eco);
        case NodeRole.VarAssignment:
            return evaluateVarAssignment(statement as VarAssignment, eco);
        default:
            console.error(`Statement cannot be interpreted atm, Sorry!: ${statement.kind}`);
            process.exit();
    }
}