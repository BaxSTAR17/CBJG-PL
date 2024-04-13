Environment = { parent=nil, variables={}, functions={}, constants={} }
function Environment.new(parent)
    self.parent = parent
    self.functions = {
        ["helloCarlos"] = print
    }
    self.constants = {}
    self.variables = {}
    return self
end

function Environment.callvar(environment, name)
    local env = Environment.findhome(environment, name)
    if env ~= nil and env.variables ~= nil and env.variables[name] ~= nil then
        return env.variables[name]
    elseif env ~= nil and env.constants ~= nil and env.constants[name] ~= nil then
        return env.constants[name]
    else error("Variable "..name.." doesn't exist!") end
end

function Environment.declarevar(env, access, name, value)
    if access == "DeclareRock" then
        if env.variables ~= nil and env.variables[name] ~= nil then error("Variable "..name.." already declared!") end
        env.variables[name] = value
    elseif access == "ConstRock" then
        if env.constants ~= nil and env.constants[name] ~= nil then error("Variable "..name.." already declared!") end
        env.constants[name] = value
    else error("Can't declare variable with: "..access) end
    return value
end

function Environment.assignvar(environment, name, value)
    local env = environment.findhome(name)
    if env.constants ~= nil and env.constants[name] ~= nil then error("Can't assign constant variable "..name) end
    env.variables[name] = value
    return value
end

function Environment.callfunction(env, fname, fparams)
    if env.functions[fname] == nil then error("Can't find function "..fname) end
    if type(fparams) == "table" then
        for i = 1, #fparams do
            env.functions[fname](fparams[i])
        end
    else env.functions[fname](fparams)
    end
    return fname
end

function Environment.findhome(env, name)
    if env ~= nil and env.parent ~= nil and (env.parent.variables[name] ~= nil or env.parent.constants[name] ~= nil) then
        return env.parent
    elseif env ~= nil and (env.constants ~= nil and env.variables ~= nil) and env.variables[name] ~= nil or env.constants[name] ~= nil and env ~= nil then
        return env
    else error("Variable "..name.." doesn't exist!") end
end

function evaluate(exps)
    local env = Environment.new(nil)
    local result = 0
    for i=1, #exps do result = evaluateprimary(exps[i], env) end
    return result
end

function evaluateprimary(exp, env)
    if exp.node == "NumberLiteral" or exp.node == "StringLiteral" then
        return exp.left
    elseif exp.node == "Identifier" then
        return Environment.callvar(env, exp.left)
    elseif exp.node == "DeclareExpression" then
        return Environment.declarevar(env, exp.addon, exp.left.left, evaluateprimary(exp.right))
    elseif exp.node == "AssignExpression" then
        return Environment.assignvar(env, exp.left, evaluateprimary(exp.right))
    elseif exp.node == "FunctionCall" then
        return evaluatefunctioncall(exp, env)
    elseif exp.node == "BinaryExpression" then
        return evaluatebinary(exp, env)
    else error("Can't evaluate expression of node: "..exp.node) end
end

function evaluatebinary(exp, env)
    local left = evaluateprimary(exp.left, env)
    local right = evaluateprimary(exp.right, env)
    if type(left) == "number" and type(right) == "number" and exp.operator == "PlusRock" then
        return left + right
    elseif type(left) == "number" and type(right) == "number" and exp.operator == "MinusRock" then
        return left - right
    elseif type(left) == "number" and type(right) == "number" and exp.operator == "MultRock" then
        return left * right
    elseif type(left) == "number" and type(right) == "number" and exp.operator == "DivRock" then
        return left / right
    elseif type(left) == "number" and type(right) == "number" and exp.operator == "ModRock" then
        return left % right
    elseif type(left) == "string" and type(right) == "string" and exp.operator == "PlusRock" then
        return left .. right
    elseif type(left) == "string" and type(right) == "string" and exp.operator == "MinusRock" then
        return left:gsub(right, "")
    else error("Can't evaluate binary expression") end
end

function evaluatefunctioncall(exp, env)
    local fparams = {}
    if type(exp.right) == "table" then for i = 1, #exp.right do fparams[i] = evaluateprimary(exp.right[i], env) end
    else error("Can't parse function arguments") end
    return Environment.callfunction(env, exp.left.left, fparams)
end