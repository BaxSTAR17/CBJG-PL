package language;

public class RuntimeResult {
	public enum Bearing {
		Number,
		String,
		Boolean,
		Null
	}
	
	public Bearing bearing;
	public int number;
	public String string;
	public Boolean bool;
	public RuntimeResult(Bearing bearing) { this.bearing = bearing; }
}
