from lexer import Lexer
from parsel import Parsel

class Eval:
    def __init__(self, AST):
        self.AST = AST
        self.fname = []
        self.fblock = []

    def run(self, node):
        if isinstance(node, list):
            for n in node:
                for k, v in n.items():
                    self.execute([k, v])
            
        elif isinstance(node, dict):
            for k, v in node.items():
                self.execute([k, v])

    def execute(self, loc):
        if isinstance(loc[1], list):
            self.run(loc[1])
        elif loc[0] == 'helloCarlos':
            self.helloCarlos(loc[1])
        elif loc[0] == 'joaquit':
            self.stop()
        elif loc[0] == 'joshua':
            self.fname.append(loc[1])
        elif loc[0] == 'function block':
            lexer = Lexer(loc[1])
            lexer.tokenizer()
            parsel = Parsel(lexer.tokens)
            parsel.runAST()
            for n in parsel.AST:
                for k, v in n.items():
                    self.fblock.append(v)
                    
        elif loc[0] == self.fname[0]:
            func = f'def {self.fname[0]}(): print("{self.fblock[0]}")'
            tion = f'{self.fname[0]}()'
            exec(func)
            exec(tion)

    def helloCarlos(self, v):
        print(v)

    def stop(self):
        quit()