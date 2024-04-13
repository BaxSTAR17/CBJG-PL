package language;

import language.*;

public class Main {

	public static void main(String[] args) throws Exception {
		java.io.File file = new java.io.File("D:/Downloads 2/CODING EXPERIMENTATIONS/carlos/Java/language/src/language/hello.cbjg");
		java.io.BufferedReader buffer = new java.io.BufferedReader(new java.io.FileReader(file));
		String code = "", line = "";
		while((line = buffer.readLine()) != null) code += line;
		buffer.close();
		Lexer lexer = new Lexer();
		lexer.harvestPlants(code.trim());
		Parser parser = new Parser(lexer.plants);
		Environment environment = new Environment();
		for(int i = 0; parser.ast.arr[i].node != language.Expression.SendNodes.EndFile; i++) Interpreter.evaluate(parser.ast.arr[i], environment);
	}

}
