package language;

public class Expression {
	public static enum SendNodes {
		NumberLiteral,
		StringLiteral,
		BinaryExpression,
		DeclareExpression,
		AssignExpression,
		Identifier,
		FunctionCall,
		EndFile
	}
	public SendNodes node;
	
	public Expression(SendNodes node) { this.node = node; }
	
	public int num = 0;
	public String string = "";
	public String name = "";
	public BinaryExpression binaryexp;
	public DeclareExpression declareexp;
	public AssignExpression assignexp;
	public FunctionCall functioncall;
	
	public class BinaryExpression {
		public Expression left;
		public language.Lexer.Seed operator;
		public Expression right;
		public BinaryExpression(Expression left, language.Lexer.Seed operator, Expression right) { this.left = left; this.operator = operator; this.right = right; }
	}
	
	public class DeclareExpression {
		public String name;
		public Boolean constant;
		public Boolean access;
		public Expression value;
		public DeclareExpression(String name, Boolean constant, Boolean access, Expression value) { this.name = name; this.constant = constant; this.access = access; this.value = value; }
	}
	
	public class AssignExpression {
		public String name;
		public Expression value;
		public AssignExpression(String name, Expression value) { this.name = name; this.value = value; }
	}
	
	public class FunctionCall {
		public String fname;
		public language.Vector<Expression> fparams = new language.Vector<language.Expression>(Expression.class, 5);
		public FunctionCall(String fname, language.Vector<Expression> fparams) { this.fname = fname; this.fparams = fparams; }
	}
}
