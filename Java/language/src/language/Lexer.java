package language;

public class Lexer {
	public static enum Seed {
		NumberSeed,
		StringSeed,
		IdentifierSeed,
		OpenParent,
		CloseParent,
		EqualsSeed,
		PlusOperateSeed,
		MinusOperateSeed,
		MultOperateSeed,
		DivOperateSeed,
		ModOperateSeed,
		AutoDeclareSeed,
		ConstDeclareSeed,
		PublicDeclare,
		PrivateDeclare,
		EndLine,
		EndFile
	}
	
	public static class Plant { 
		public Lexer.Seed seed;
		public String fruit;
		
		public Plant(Lexer.Seed seed, String fruit) { this.seed = seed; this.fruit = fruit; }
	}
	
	public static java.util.HashMap<String, Lexer.Seed> keywords = new java.util.HashMap<String, Lexer.Seed>();
	public language.Vector<Plant> plants;
	
	public language.Vector<Plant> harvestPlants(String src) throws Exception {
		Vector<Plant> plants = new language.Vector<Plant>(Plant.class, 5);
		this.keywords.put("Ghen", Lexer.Seed.PlusOperateSeed);
		this.keywords.put("gHen", Lexer.Seed.MinusOperateSeed);
		this.keywords.put("ghEn", Lexer.Seed.MultOperateSeed);
		this.keywords.put("gheN", Lexer.Seed.DivOperateSeed);
		this.keywords.put("GHEN", Lexer.Seed.ModOperateSeed);
		this.keywords.put("ghen", Lexer.Seed.EqualsSeed);
		this.keywords.put("car", Lexer.Seed.AutoDeclareSeed);
		int i = 0;
		while(i < src.length()) {
			if(src.charAt(i) == '(') plants.insert(Plant.class, new Lexer.Plant(Lexer.Seed.OpenParent, String.valueOf(src.charAt(i))));
			else if(src.charAt(i) == ')') plants.insert(Plant.class, new Lexer.Plant(Lexer.Seed.CloseParent, String.valueOf(src.charAt(i))));
			else if(src.charAt(i) == '=') plants.insert(Plant.class, new Lexer.Plant(Lexer.Seed.EqualsSeed, String.valueOf(src.charAt(i))));
			else {
				String fruitstring = "";
				if(src.charAt(i) == '"') {
					i++;
					while(i < src.length() && src.charAt(i) != '"') {
						fruitstring += src.charAt(i);
						i++;
						if(i < src.length() && src.charAt(i) == '"') break;
					}
					i++;
					plants.insert(Plant.class, new Lexer.Plant(Lexer.Seed.StringSeed, fruitstring)); 
					continue;
				}
				else if(Character.isDigit(src.charAt(i))) {
					fruitstring += src.charAt(i);
					while(i < src.length() && Character.isDigit(src.charAt(i))) { 
						i++; 
						if(i < src.length() && Character.isDigit(src.charAt(i))) fruitstring += src.charAt(i); 
					}
					plants.insert(Plant.class, new Lexer.Plant(Lexer.Seed.NumberSeed, fruitstring));
					continue;
				}
				else if(Character.isAlphabetic(src.charAt(i))) {
					fruitstring += src.charAt(i);
					while(i < src.length() && Character.isLetterOrDigit(src.charAt(i)) && !this.keywords.containsKey(fruitstring)) {
						i++;
						if(i < src.length() && Character.isLetterOrDigit(src.charAt(i)) && !this.keywords.containsKey(fruitstring)) fruitstring += src.charAt(i);
					}
					if(this.keywords.containsKey(fruitstring)) {
						plants.insert(Plant.class, new Lexer.Plant(this.keywords.get(fruitstring), fruitstring));
						i++;
					}
					else plants.insert(Plant.class, new Lexer.Plant(Lexer.Seed.IdentifierSeed, fruitstring));
					continue;
				}
				else if(src.charAt(i) == ' ' || src.charAt(i) == '\n' || src.charAt(i) == '\t' || src.charAt(i) == '\r') { i++; continue; }
				else throw new Exception("Unexpected word was found"+src.charAt(i));
			}
			i++;
		}
		plants.insert(Plant.class, new Lexer.Plant(Lexer.Seed.EndFile, "EOF"));
		this.plants = plants;
		return plants;
	}
}
