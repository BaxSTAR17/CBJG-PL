Ore = { rock=nil, mineral=nil }
local keywords = {
    ["Ghen"] = "PlusRock",
    ["gHen"] = "MinusRock",
    ["ghEn"] = "MultRock",
    ["gheN"] = "DivRock",
    ["GHEN"] = "ModRock",
    ["ghen"] = "EqualsRock",
    ["car"] = "DeclareRock",
    ["LOS"] = "ConstRock",
}

function Ore.new(rock, mineral)
    local self = setmetatable({}, Ore)
    self.rock = rock
    self.mineral = mineral
    return self
end

function minerocks(code) 
    local ores = {}
    local oreindex = 1
    local i = 1
    while i <= #code do
        if code:sub(i, i) == "(" then ores[oreindex] = Ore.new("OpenParent", "(") oreindex = oreindex+1
        elseif code:sub(i, i) == ")" then ores[oreindex] = Ore.new("CloseParent", ")") oreindex = oreindex+1
        elseif code:sub(i, i) == "=" then ores[oreindex] = Ore.new("EqualsRock", "=") oreindex = oreindex+1
        elseif code:sub(i, i) == "," then ores[oreindex] = Ore.new("CommaRock", ",") oreindex = oreindex+1
        else
            local orestring = ""
            if code:sub(i, i) == "\"" then
                i=i+1
                while code:sub(i, i) ~= "\"" and i <= #code do orestring=orestring..code:sub(i, i) i=i+1 end
                ores[oreindex] = Ore.new("StringRock", orestring)
                oreindex = oreindex+1
            elseif code:sub(i, i):match("%d") then
                while code:sub(i, i):match("%d") and i <= #code do orestring=orestring..code:sub(i, i) i=i+1 end
                ores[oreindex] = Ore.new("NumberRock", orestring)
                oreindex = oreindex+1
                i=i-1
            elseif code:sub(i, i):match("%w") then
                while code:sub(i, i):match("%w") and keywords[orestring] == nil and i <= #code do orestring=orestring..code:sub(i, i) i=i+1 end
                if keywords[orestring] then ores[oreindex] = Ore.new(keywords[orestring], orestring)
                else ores[oreindex] = Ore.new("Identifier", orestring) end
                oreindex = oreindex+1
                i=i-1
            else code:sub(i, i):match("%s")
            end
        end
        i=i+1
    end
    ores[oreindex] = Ore.new("EndFile", "EOF")
    return ores
end