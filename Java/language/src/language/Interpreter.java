package language;

public class Interpreter {
	public static language.RuntimeResult evaluate(language.Expression expression, language.Environment env) throws Exception {
		switch(expression.node) {
			case NumberLiteral:
				language.RuntimeResult num = new RuntimeResult(language.RuntimeResult.Bearing.Number);
				num.number = expression.num;
				return num;
			case StringLiteral:
				language.RuntimeResult res = new RuntimeResult(language.RuntimeResult.Bearing.String);
				res.string = expression.string;
				return res;
			case BinaryExpression:
				return evaluateBinaryExpression(expression, env);
			case DeclareExpression:
				return evaluateDeclareExpression(expression, env);
			case AssignExpression:
				return evaluateAssignExpression(expression, env);
			case Identifier:
				return env.callVariable(expression.name);
			case FunctionCall:
				return evaluateFunctionCall(expression, env);
			default:
				throw new Exception("Can't evaluate expression found: "+expression.node);
		}
	}
	
	public static language.RuntimeResult evaluateDeclareExpression(language.Expression expression, language.Environment env) throws Exception {
		language.RuntimeResult value = evaluate(expression.declareexp.value, env);
		return env.declareVariable(expression.declareexp.name, value);
	}
	
	public static language.RuntimeResult evaluateAssignExpression(language.Expression expression, language.Environment env) throws Exception {
		language.RuntimeResult value = evaluate(expression.assignexp.value, env);
		return env.assignVariable(expression.assignexp.name, value);
	}
	
	public static language.RuntimeResult evaluateFunctionCall(language.Expression expression, language.Environment env) throws Exception {
		language.Vector<language.RuntimeResult> fparams = new language.Vector<language.RuntimeResult>(language.RuntimeResult.class, 5);
		for(int i = 0; expression.functioncall.fparams.arr[i] != null; i++) fparams.insert(language.RuntimeResult.class, evaluate(expression.functioncall.fparams.arr[i], env));
		return env.callFunction(expression.functioncall.fname, fparams);
	}
	
	public static language.RuntimeResult evaluateBinaryExpression(language.Expression expression, language.Environment env) throws Exception {
		if(expression.binaryexp.left.node == language.Expression.SendNodes.StringLiteral || expression.binaryexp.right.node == language.Expression.SendNodes.StringLiteral) throw new Exception("Strings aren't supported yet for binary expressions!");
		language.RuntimeResult res = new RuntimeResult(language.RuntimeResult.Bearing.Number);
		switch(expression.binaryexp.operator) {
			case PlusOperateSeed:
				res.number = evaluate(expression.binaryexp.left, env).number + evaluate(expression.binaryexp.right, env).number;
				return res;
			case MinusOperateSeed:
				res.number = evaluate(expression.binaryexp.left, env).number - evaluate(expression.binaryexp.right, env).number;
				return res;
			case MultOperateSeed:
				res.number = evaluate(expression.binaryexp.left, env).number * evaluate(expression.binaryexp.right, env).number;
				return res;
			case DivOperateSeed:
				res.number = evaluate(expression.binaryexp.left, env).number / evaluate(expression.binaryexp.right, env).number;
				return res;
			case ModOperateSeed:
				res.number = evaluate(expression.binaryexp.left, env).number % evaluate(expression.binaryexp.right, env).number;
				return res;
			default:
				throw new Exception("Cannot evaluate binary expression with operator: "+ expression.binaryexp.operator);
		}
	}
 }
