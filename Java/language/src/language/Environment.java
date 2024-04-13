package language;

import java.lang.reflect.Array;

public class Environment {
	public Environment parent;
	public language.Vector<String> globalFunctions;
	public java.util.HashMap<String, Integer> numberGarden;
	public java.util.HashMap<String, String> stringGarden;
	public java.util.HashMap<String, Boolean> booleanGarden;
	public Environment child;
	
	public Environment() {
		this.globalFunctions = new Vector<String>(String.class, 3);
		this.globalFunctions.insert(String.class, "helloCarlos");
		this.numberGarden = new java.util.HashMap<String, Integer>();
		this.stringGarden = new java.util.HashMap<String, String>();
		this.booleanGarden = new java.util.HashMap<String, Boolean>();
	}
	
	public language.RuntimeResult declareVariable(String varname, language.RuntimeResult value) throws Exception {
		if(this.numberGarden.containsKey(varname) || this.stringGarden.containsKey(varname) || this.booleanGarden.containsKey(varname)) throw new Exception("Variable already declared");
		switch(value.bearing) {
			case Number:
				this.numberGarden.put(varname, value.number);
				return value;
			case String:
				this.stringGarden.put(varname, value.string);
				return value;
			case Boolean:
				this.booleanGarden.put(varname, value.bool);
				return value;
			default:
				throw new Exception("Can't declare variable type found: "+value.bearing);
		}
	}
	
	public language.RuntimeResult assignVariable(String varname, language.RuntimeResult value) throws Exception {
		if(this.numberGarden.containsKey(varname) || this.stringGarden.containsKey(varname) || this.booleanGarden.containsKey(varname)) {
			switch(value.bearing) {
				case Number:
					this.numberGarden.put(varname, value.number);
					return value;
				case String:
					this.stringGarden.put(varname, value.string);
					return value;
				case Boolean:
					this.booleanGarden.put(varname, value.bool);
					return value;
				default:
					throw new Exception("Can't declare variable type found: "+value.bearing);
			}
		}
		else throw new Exception("Variable "+varname+" hasn't been declared yet!");
	}
	
	public language.RuntimeResult callVariable(String varname) throws Exception {
		Environment env = this.findHome(varname);
		if(env.numberGarden.containsKey(varname)) {
			language.RuntimeResult num = new RuntimeResult(language.RuntimeResult.Bearing.Number);
			num.number = env.numberGarden.get(varname);
			return num;
		}
		else if(env.stringGarden.containsKey(varname)) {
			language.RuntimeResult str = new RuntimeResult(language.RuntimeResult.Bearing.String);
			str.string = env.stringGarden.get(varname);
			return str;
		}
		else if(env.booleanGarden.containsKey(varname)) {
			language.RuntimeResult bool = new RuntimeResult(language.RuntimeResult.Bearing.Boolean);
			bool.bool = env.booleanGarden.get(varname);
			return bool;
		}
		else throw new Exception("Variable does not exist!: "+varname);
	}
	
	public language.RuntimeResult callFunction(String fname, language.Vector<language.RuntimeResult> fparams) throws Exception {
		if(this.globalFunctions.contains(String.class, fname)) throw new Exception("Function name doesn't exist: "+fname);
		switch(fname) {
			case "helloCarlos":
				return printOutput(fparams);
			default:
				throw new Exception("Function name doesn't exist: "+fname);
		}
	}
	
	public language.RuntimeResult printOutput(language.Vector<language.RuntimeResult> res) throws Exception {
		for(int i = 0; res.arr[i] != null; i++) {
			switch(res.arr[i].bearing) {
				case Number:
					System.out.println(""+res.arr[i].number);
					return res.arr[0];
				case String:
					System.out.println(res.arr[i].string);
					return res.arr[0];
				case Boolean:
					System.out.println(""+res.arr[i].bool);
					return res.arr[0];
				default:
					throw new Exception("Can't print result type of: "+res.arr[i].number);
			}
		}
		return res.arr[0];
	}
	
	public Environment findHome(String varname) throws Exception {
		if((this.parent != null) && (this.parent.numberGarden.containsKey(varname) || this.parent.stringGarden.containsKey(varname) || this.parent.booleanGarden.containsKey(varname))) return this.parent;
		else if(this.numberGarden.containsKey(varname) || this.stringGarden.containsKey(varname) || this.booleanGarden.containsKey(varname)) return this;
		else if((this.parent != null) && (this.child.numberGarden.containsKey(varname) || this.child.stringGarden.containsKey(varname) || this.child.booleanGarden.containsKey(varname))) return this.child;
		else throw new Exception("Can't find home of variable: "+varname); 
	}
}
