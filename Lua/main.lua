require "lexer"
require "parser"
require "interpreter"

src = io.open("./hello.cbjg", "r")
code = src:read("*all")
rocks = minerocks(code)
parses = parserock(rocks)
evaluate(parses)
