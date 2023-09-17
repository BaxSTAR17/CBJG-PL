from lexer import Lexer
from parsel import Parsel
from evalist import Eval

def main():
    filename = 'main.carlos'
    file     = open(filename, 'r')
    lexer = Lexer(file)
    parsel = Parsel(lexer.tokens)
    evaluator = Eval(parsel.AST)

    lexer.tokenizer()

    parsel.runAST()

    evaluator.run(parsel.AST)
    

if __name__ == '__main__':
    main()