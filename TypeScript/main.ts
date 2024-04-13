import Parser from './parser.ts';
import { evaluateProgram, NumberResult, StringResult, BooleanResult, RuntimeResult } from './interpreter.ts';
import * as read from "readline";
import { readFileSync } from "fs";
import Ecosystem from './environment.ts';

main();

function main() {
    // let rl = read.createInterface({ input: process.stdin, output: process.stdout });
    // console.log("CBJG v.1.0.0\n");
    // rl.question("cbjg - ", input);
    let src = readFileSync("./hello.cbjg", 'utf-8')
    input(src);
}

function input(src: string) {
    const parser = new Parser();
    const eco = new Ecosystem();
    if(!src || src == "exit") process.exit();
    const program = parser.createAST(src);
    const result: RuntimeResult = evaluateProgram(program, eco) as RuntimeResult;
    process.exit();
}