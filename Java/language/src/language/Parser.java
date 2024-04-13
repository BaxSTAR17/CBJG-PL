package language;

public class Parser {
	public language.Vector<language.Expression> ast;
	private int i;
	
	public Parser(language.Vector<language.Lexer.Plant> plants) throws Exception {
		this.i = 0;
		language.Vector<language.Expression> ast = new language.Vector<language.Expression>(language.Expression.class, 5);
		while(plants.arr[this.i] != null && plants.arr[this.i].seed != language.Lexer.Seed.EndFile) { ast.insert(language.Expression.class, this.parsePlant(plants)); }
		ast.insert(language.Expression.class, new Expression(language.Expression.SendNodes.EndFile));
		this.ast = ast;
	}
	
	public language.Expression parsePlant(language.Vector<language.Lexer.Plant> plants) throws Exception {
		switch(plants.arr[this.i].seed) {
			case ConstDeclareSeed:
			case AutoDeclareSeed:
				return parseDeclareExp(plants);
			default:
				return parseAssignExp(plants);
		}
	}
	
	public language.Expression parseAssignExp(language.Vector<language.Lexer.Plant> plants) throws Exception {
		language.Expression left = parseAdditiveExp(plants);
		if(left.node == language.Expression.SendNodes.Identifier && plants.arr[this.i] != null && plants.arr[this.i].seed == language.Lexer.Seed.EqualsSeed) {
			language.Expression ass = new Expression(language.Expression.SendNodes.AssignExpression);
			this.i++;
			language.Expression value = parsePlant(plants);
			ass.assignexp = ass.new AssignExpression(left.name, value);
			return ass;
		}
		return left;
	}
	
	public language.Expression parseDeclareExp(language.Vector<language.Lexer.Plant> plants) throws Exception {
		Boolean constant = plants.arr[this.i].seed == language.Lexer.Seed.AutoDeclareSeed ? false : true;
		this.i++;
		//do accessibility handling later
		if(plants.arr[this.i] == null || plants.arr[this.i].seed != language.Lexer.Seed.IdentifierSeed) throw new Exception("Expected Identifier but found: "+ plants.arr[this.i].seed);
		String name = plants.arr[this.i].fruit;
		this.i++;
		if(plants.arr[this.i] == null || plants.arr[this.i].seed != language.Lexer.Seed.EqualsSeed) throw new Exception("Expected Assignment but found: "+ plants.arr[this.i].seed);
		this.i++;
		language.Expression value = parsePlant(plants);
		language.Expression dec = new Expression(language.Expression.SendNodes.DeclareExpression);
		dec.declareexp = dec.new DeclareExpression(name, constant, true, value);
		return dec;
	}
	
	public language.Expression parseAdditiveExp(language.Vector<language.Lexer.Plant> plants) throws Exception {
		language.Expression left = parseMultiplicativeExp(plants);
		if(plants.arr[this.i] != null) {
			while(plants.arr[this.i].seed == language.Lexer.Seed.PlusOperateSeed || plants.arr[this.i].seed == language.Lexer.Seed.MinusOperateSeed) {
				language.Lexer.Seed operator = plants.arr[this.i].seed;
				this.i++;
				language.Expression right = parseMultiplicativeExp(plants);
				language.Expression bin = new Expression(language.Expression.SendNodes.BinaryExpression);
				bin.binaryexp = bin.new BinaryExpression(left, operator, right);
				return bin;
			}
		}
		return left;
	}
	
	public language.Expression parseMultiplicativeExp(language.Vector<language.Lexer.Plant> plants) throws Exception {
		language.Expression left = parseFunctionCall(plants);
		if(plants.arr[this.i] != null) {
			while(plants.arr[this.i].seed == language.Lexer.Seed.MultOperateSeed || plants.arr[this.i].seed == language.Lexer.Seed.DivOperateSeed || plants.arr[this.i].seed == language.Lexer.Seed.ModOperateSeed) {
				language.Lexer.Seed operator = plants.arr[this.i].seed;
				this.i++;
				language.Expression right = parseFunctionCall(plants);
				language.Expression bin = new Expression(language.Expression.SendNodes.BinaryExpression);
				bin.binaryexp = bin.new BinaryExpression(left, operator, right);
				return bin;
			}
		}
		return left;	
	}
	
	public language.Expression parseFunctionCall(language.Vector<language.Lexer.Plant> plants) throws Exception {
		language.Expression left = parsePrimaryPlant(plants);
		if(left.node == language.Expression.SendNodes.Identifier && plants.arr[this.i] != null && plants.arr[this.i].seed == language.Lexer.Seed.OpenParent) {
			String fname = left.name;
			language.Vector<language.Expression> fparams = new language.Vector<language.Expression>(language.Expression.class, 5);
			this.i++;
			while(plants.arr[this.i].seed != language.Lexer.Seed.CloseParent && plants.arr[this.i].seed != language.Lexer.Seed.EndFile) {
				fparams.insert(language.Expression.class, parsePlant(plants));
				this.i++;
			}
			this.i++;
			language.Expression fcall = new Expression(language.Expression.SendNodes.FunctionCall);
			fcall.functioncall = fcall.new FunctionCall(fname, fparams);
			return fcall;
		}
		return left;
	}
	
	public language.Expression parsePrimaryPlant(language.Vector<language.Lexer.Plant> plants) throws Exception {
		switch(plants.arr[this.i].seed) {
			case NumberSeed:
				language.Expression num = new Expression(language.Expression.SendNodes.NumberLiteral);
				num.num = Integer.parseInt(plants.arr[this.i].fruit);
				this.i++;
				return num;
			case IdentifierSeed:
				language.Expression id = new Expression(language.Expression.SendNodes.Identifier);
				id.name = plants.arr[this.i].fruit;
				this.i++;
				return id;
			case StringSeed:
				language.Expression string = new Expression(language.Expression.SendNodes.StringLiteral);
				string.string = plants.arr[this.i].fruit;
				this.i++;
				return string;
			default:
				throw new Exception("Can't parse primary word found: "+plants.arr[this.i].fruit);
		}
	}
}
