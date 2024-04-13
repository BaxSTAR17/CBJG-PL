BoulderExpression = { node=nil, addon=nil, left=nil, operator=nil, right=nil }
function BoulderExpression.new(node, addon, left, operator, right)
    self = setmetatable({}, BoulderExpression)
    self.node = node
    self.addon = addon
    self.left = left
    self.operator = operator
    self.right = right
    return self
end

local currentrock = 1

function parserock(rocks)
    local boulders = {}
    local boulderindex = 1
    while currentrock < #rocks do
        if rocks[currentrock].rock == "DeclareRock" or rocks[currentrock].rock == "ConstRock" then boulders[boulderindex] = parsedeclare(rocks) boulderindex = boulderindex + 1
        else boulders[boulderindex] = parseassignrock(rocks) boulderindex = boulderindex + 1 end
        currentrock = currentrock + 1
        --print("Current Index: "..currentrock)
    end
    return boulders
end

function parsedeclare(rocks)
    local addon = rocks[currentrock].rock
    currentrock = currentrock + 1
    local left = parseadditiverock(rocks)
    currentrock = currentrock + 1
    if currentrock < #rocks and left.node == "Identifier" and rocks[currentrock].rock == "EqualsRock" then
        local operator = rocks[currentrock].rock
        currentrock = currentrock + 1
        local right = parseadditiverock(rocks)
        return BoulderExpression.new("DeclareExpression", addon, left, operator, right)
    else error("can't parse declare expression!")
    end
end

function parseassignrock(rocks)
    local left = parseadditiverock(rocks)
    if currentrock < #rocks and left.node == "Identifier" and rocks[currentrock+1].rock == "EqualsRock" then
        currentrock = currentrock + 1
        local operator = rocks[currentrock].rock
        currentrock = currentrock + 1
        local right = parseadditiverock(rocks)
        return BoulderExpression.new("AssignExpression", nil, left, operator, right)
    else return left
    end
end

function parseadditiverock(rocks)
    local left = parsemultiplicativerocks(rocks)
    if (currentrock < #rocks) and (rocks[currentrock+1].rock == "PlusRock" or rocks[currentrock+1].rock == "MinusRock") then
        currentrock = currentrock + 1
        local operator = rocks[currentrock].rock
        currentrock = currentrock + 1
        local right = parsemultiplicativerocks(rocks)
        return BoulderExpression.new("BinaryExpression", nil, left, operator, right)
    else return left
    end
end

function parsemultiplicativerocks(rocks)
    local left = parsefunctioncall(rocks)
    if (currentrock < #rocks) and (rocks[currentrock+1].rock == "MultRock" or rocks[currentrock+1].rock == "DivRock" or rocks[currentrock+1].rock == "ModRock") then
        currentrock = currentrock + 1
        local operator = rocks[currentrock].rock
        currentrock = currentrock + 1
        local right = parsefunctioncall(rocks)
        return BoulderExpression.new("BinaryExpression", nil, left, operator, right)
    else return left
    end
end

function parsefunctioncall(rocks)
    local left = parseprimaryrock(rocks)
    local fparams = {}
    local paramnum = 1
    if currentrock < #rocks and left.node == "Identifier" and rocks[currentrock+1].rock == "OpenParent" then
        currentrock = currentrock + 1
        currentrock = currentrock + 1
        while currentrock < #rocks and rocks[currentrock].rock ~= "CloseParent" do
            local currentparam = parseassignrock(rocks)
            fparams[paramnum] = currentparam
            paramnum = paramnum + 1
            currentrock = currentrock + 1
            if rocks[currentrock].rock == "CommaRock" then currentrock = currentrock + 1 
            elseif rocks[currentrock].rock == "CloseParent" then break
            else error("Can't parse function call, missing comma or parentheses") end
        end
        if rocks[currentrock].rock == "CloseParent" then currentrock = currentrock + 1
        else error("Missing pair of parentheses") end
        return BoulderExpression.new("FunctionCall", nil, left, "OpenParent", fparams)
    else return left
    end
end

function parseprimaryrock(rocks)
    if rocks[currentrock].rock == "NumberRock" then return BoulderExpression.new("NumberLiteral", nil, tonumber(rocks[currentrock].mineral), nil, nil) 
    elseif rocks[currentrock].rock == "StringRock" then return BoulderExpression.new("StringLiteral", nil, rocks[currentrock].mineral, nil, nil) 
    elseif rocks[currentrock].rock == "Identifier" then return BoulderExpression.new("Identifier", nil, rocks[currentrock].mineral, nil, nil) 
    else error("Can't parse primary word:"..rocks[currentrock].rock) end
end