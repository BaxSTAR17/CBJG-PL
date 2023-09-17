class Eval:
    def __init__(self, AST):
        self.AST = AST

    def run(self, node):
        if isinstance(node, list):
            for n in node:
                for k, v in n.items():
                    self.execute([k, v])
            
        elif isinstance(node, dict):
            for k, v in node.iteritems():
                self.execute([k, v])

    def execute(self, loc):
        if isinstance(loc[1], list):
            self.run(loc[1])
        elif loc[0] == 'helloCarlos':
            self.helloCarlos(loc[1])
        elif loc[0] == 'joaquit':
            self.stop()
        
    def helloCarlos(self, v):
        print(v)

    def stop(self):
        quit()