import { NodeRole, Program, Statement, Expression, BinaryExpression, Identifier, NumericLiteral, NullVoid, FunctionCall, VarDeclaration, VarAssignment, StringLiteral } from "./ast.ts";
import { Role, analyze, Word } from "./lexer.ts"

//Presidence
//AssignmentExpression
//MemberExpression
//FunctionCall
//LogicalExpression
//ComparisonExpression
//AdditiveExpression
//MultiplicativeExpression
//UnaryExpression/Function Call Expression
//PrimaryExpression

export default class Parser {
    private words: Word[] = [];
    private currentWord(): Word {
        return this.words[0] as Word;
    }
    private movWord(): Word {
        return this.words.shift() as Word;
    }
    private parseWord(): Statement {
        switch(this.currentWord().role) {
            case Role.LetDeclare:
            case Role.ConstDeclare:
                return this.parseVarDeclare();
            default:
                return this.parseAssignExp();
        }
    }
    private expect(role: Role, message: string): Word {
        if(this.currentWord().role == role) return this.movWord();
        console.error(message);
        process.exit();
    }
    private parseAssignExp(): Expression {
        const left = this.parseAdditiveExp();
        while(left.kind == NodeRole.Identifier && this.currentWord().role == Role.Equals) {
            this.movWord();
            const right = this.parseAdditiveExp();
            let result = {
                kind: NodeRole.VarAssignment,
                identifier: (left as Identifier).string,
                value: right
            } as VarAssignment
            return result;
        }
        return left;
    }
    private parseVarDeclare(): Expression {
        const isconst: boolean  = this.movWord().role == Role.ConstDeclare;
        const id = this.expect(Role.Identifier, "Expected variable/function name!");
        let value: Expression = { kind: NodeRole.NullLiteral, value: "null" } as NullVoid;
        if(this.currentWord().role == Role.EndLine && !isconst) value = { kind: NodeRole.NullLiteral, value: "null" } as NullVoid;
        else if(this.currentWord().role == Role.Equals) {
            this.movWord();
            value = this.parseWord();
        }
        else throw this.currentWord().role == Role.EndLine ? "Constant Variable must be assigned" : `Variable ${id} declaration is leaking`;
        return { kind: NodeRole.VarDeclaration, constant: isconst, identifier: id.value, value: value } as VarDeclaration;
    }
    private parseAdditiveExp(): Expression {
        let left = this.parseMultiplicativeExp();
        while(this.currentWord().role == Role.PlusOperator || this.currentWord().role == Role.MinusOperator) {
            const operator = this.movWord().value;
            this.currentWord();
            const right = this.parseMultiplicativeExp();
            if(left.kind == NodeRole.StringLiteral || right.kind == NodeRole.StringLiteral) throw "Invalid Binary expression, one of them is a string!";
            left = {
                kind: NodeRole.BinaryExpression,
                left: left,
                right: right,
                operator: operator
            } as BinaryExpression;
        }
        return left;
    }
    private parseMultiplicativeExp(): Expression {
        let left = this.parseFunctionCall();
        while(this.currentWord().role == Role.MultOperator || this.currentWord().role == Role.DivOperator || this.currentWord().role == Role.ModOperator) {
            const operator = this.movWord().value;
            const right = this.parseFunctionCall();
            if(left.kind == NodeRole.StringLiteral || right.kind == NodeRole.StringLiteral) throw "Invalid Binary expression, one of them is a string!";
            left = {
                kind: NodeRole.BinaryExpression,
                left: left,
                right: right,
                operator: operator
            } as BinaryExpression;
        }
        return left;
    }
    private parseFunctionCall(): Expression {
        let left = this.parsePrimaryExp();
        if(this.currentWord().role == Role.OpenParent) {
            this.movWord();
            let func = {
                kind: NodeRole.FunctionCall,
                fname: (left as Identifier).string,
                fparams: this.parseArgumentsList()
            } as FunctionCall;
            this.expect(Role.CloseParent, "Expected ')', there is an incomplete pair of parentheses");
            return func;
        }
        return left;
    }
    private parseArgumentsList(): Expression[] {
        if(this.currentWord().role == Role.CloseParent) return [];
        const args: Expression[] = [];
        args.push(this.parseAssignExp());
        while(this.currentWord().role == Role.Comma) {
            this.movWord();
            args.push(this.parseAssignExp());
        }
        return args;
    }
    private parsePrimaryExp(): Expression {
        switch(this.currentWord().role) {
            case Role.String:
                return { kind: NodeRole.StringLiteral, string: this.movWord().value } as StringLiteral;
            case Role.Identifier:
                return { kind: NodeRole.Identifier, string: this.movWord().value } as Identifier; 
            case Role.Number:
                return { kind: NodeRole.NumericLiteral, number: parseInt(this.movWord().value) } as NumericLiteral;
            case Role.OpenParent: {
                    this.movWord();
                    const value = this.parseAdditiveExp();
                    this.expect(Role.CloseParent, "Expected ')', there is an incomplete pair of parentheses");
                    return value;
                }
            case Role.Null:
                this.movWord();
                return { kind: NodeRole.NullLiteral, value: "null" } as NullVoid;
            default:
                console.log("Unknown word was found: " + this.movWord().value);
                return {} as Statement;
        } 
    }

    public createAST(src: string): Program {
        this.words = analyze(src);
        const program: Program = {
            kind: NodeRole.Program,
            body: []
        }
        while(this.currentWord().role != Role.EndFile) program.body.push(this.parseWord());
        return program;
    }
}