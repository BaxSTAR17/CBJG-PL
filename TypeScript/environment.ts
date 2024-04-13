import { RuntimeResult, NumberResult, StringResult, NullResult, BooleanResult } from './interpreter.ts';
import { FunctionCall, Expression, NodeRole } from './ast';

export default class Ecosystem {
    private parent?: Ecosystem;
    private variables: Map<string, RuntimeResult>;
    private constants: Map<string, RuntimeResult>;
    private presetfunctions: String[] = [
        "helloCarlos"
    ];
    private functions: String[] = [];
    constructor(parent?: Ecosystem) {
        this.parent = parent;
        this.variables = new Map();
        this.constants = new Map();
    }
    public declareVar(varname: string, varval: RuntimeResult): RuntimeResult {
        if(this.variables.has(varname)) throw `The variable '${varname}' is already declared!`;
        this.variables.set(varname, varval);
        return varval;
    }
    public declareConst(varname: string, varval: RuntimeResult): RuntimeResult {
        if(this.constants.has(varname)) throw `The constant '${varname}' is already declared!`;
        this.constants.set(varname, varval);
        return varval;
    }
    public assignVar(varname: string, varval: RuntimeResult): RuntimeResult {
        const eco: Ecosystem = this.getEcosystem(varname);
        eco.variables.set(varname, varval);
        return varval;
    }
    public callVar(varname: string): RuntimeResult {
        const eco: Ecosystem = this.getEcosystem(varname);
        return eco.variables.has(varname) ? eco.variables.get(varname) as RuntimeResult : eco.constants.get(varname) as RuntimeResult;
    }
    public callFunc(fname: string, fparams: RuntimeResult[]): RuntimeResult {
        if(!this.presetfunctions.includes(fname) /*&& !this.functions.includes(statement.fname)*/) throw `The function ${fname} does not exist`;
        let param;
        while(fparams.length > 0) {
            if(fname == "helloCarlos") {
                switch(fparams[0].type) {
                    case "number":
                        console.log((fparams.shift() as NumberResult).value);
                        break;
                    case "string":
                        console.log((fparams.shift() as StringResult).value);
                        break;
                    case "bool":
                        console.log((fparams.shift() as BooleanResult).value);
                        break;
                    case "null":
                        console.log((fparams.shift() as NullResult).value);
                        break;
                }
            }
        }
        return fparams[0];
    }
    public getEcosystem(varname: string): Ecosystem {
        if(this.variables.has(varname) || this.constants.has(varname)) return this as Ecosystem;
        if(this.parent == undefined) throw `The non-constant variable '${varname}' does not exist!`;
        return this.parent.getEcosystem(varname);
    }
}