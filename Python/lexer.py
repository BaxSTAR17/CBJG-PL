class Lexer:
    def __init__(self, data):
        self.data = data
        self.tokens = []
        self.functions = [
            "helloCarlos"
        ]
        self.labels = [
            "CARLOS"
        ]

    def tokenizer(self):
        for loc in self.data:
            tmp = []
            tid = ''

            for l in loc:
                if ''.join(tmp) in self.functions:
                    self.tokens.append({'id': 'function', 'value': ''.join(tmp)})
                    tmp = []
                elif ''.join(tmp) in self.labels:
                    self.tokens.append({'id': 'label', 'value': ''.join(tmp)})
                    tmp = []
                elif l == '(' and tid == '':
                    tid = 'arguments'
                    tmp = []
                elif l == ')' and tid == 'arguments':
                    self.tokens.append({'id': tid, 'value': ''.join(tmp)})
                    tid = ''
                    tmp = []
                elif l == ' ' and tid != 'arguments':
                    continue
                else:
                    tmp.append(l)