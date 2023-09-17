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
        self.keywords = [
            "joshua"
        ]

    def tokenizer(self):
        tmp = []
        btmp = []
        tid = ''
        tl = ''
        for loc in self.data:

            for l in loc:
                if ''.join(tmp) in self.functions and tl != 'function block':
                    self.tokens.append({'id': 'function', 'value': ''.join(tmp)})
                    tid = 'arguments'
                    tmp = []
                elif ''.join(tmp) in self.keywords:
                    self.tokens.append({'id': 'keyword', 'value': ''.join(tmp)})
                    tmp = []
                    if tid == '':
                        tid = 'function name'
                elif ''.join(tmp) in self.labels:
                    self.tokens.append({'id': 'label', 'value': ''.join(tmp)})
                    tmp = []
                elif l == '(' and tid == 'arguments' and tl != 'function block':
                    tmp = []
                elif l == '(' and tid == 'function name':
                    self.tokens.append({'id': tid, 'value': ''.join(tmp)})
                    self.functions.append(''.join(tmp))
                    tid = 'arguments'
                    tmp = []
                elif l == ')' and tid == 'arguments':
                    self.tokens.append({'id': tid, 'value': ''.join(tmp)})
                    tmp = []
                elif l == '=' and tid == 'arguments':
                    tl = 'function block'
                    tid = ''
                    tmp = []
                elif l == '>' and tl == 'function block':
                    tl = 'function block'
                    tmp = []
                    btmp = []
                elif l == ';':
                    self.tokens.append({'id': tl, 'value': ''.join(btmp)})
                    tid = ''
                    tl = ''
                    tmp = []
                    btmp = []
                elif l == ' ' and tid != 'arguments':
                    continue
                elif l == ' ' and tl == 'function block':
                    btmp.append(l)
                elif l == '\n':
                    '''tmp = []
                    btmp = []'''
                else:
                    tmp.append(l)
                    btmp.append(l)