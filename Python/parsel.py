class Parsel:
    def __init__(self,  tokens):
        self.tokens = tokens
        self.AST = []

    def add_node(self, parent, node):
        for a in self.AST:
            if parent in a:
                a[parent].append(node)

    def runAST(self):
        saved = {}
        parent = {}
        collect = False

        for token in self.tokens:
            if token['id'] == 'label':
                t = {token['value']: []}

                if parent != t:
                    parent = token['value']
                    self.AST.append(t)
            
            elif token['id'] == 'function':
                if token['value'] == 'joaquit':
                    t = {token['value']: 0}
                    self.add_node(parent, t)
                else:
                    if collect == False:
                        saved = token
                        collect = True
                    else:
                        t = {saved['value']: token['value']}
                        self.add_node(parent, t)
                        collect = False

            elif token['id'] == 'arguments':
                if collect == False:
                    saved = token
                    collect = True
                else:
                    t = {saved['value']: token['value']}
                    self.add_node(parent, t)
                    collect = False
