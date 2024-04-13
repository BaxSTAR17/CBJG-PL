package language;

import java.util.*;

public class Vector<T> {
	public T[] arr;
	private int count;
	
	public Vector(Class<T> v, int length) { 
		T[] vec = (T[]) java.lang.reflect.Array.newInstance(v, length); 
		count = 0; 
		this.arr = vec;
	}
	
	public void insert(Class<T> v, T element) {
		if(arr.length == count) {
			T[] newVec = (T[]) java.lang.reflect.Array.newInstance(v, count*2);
			for(int i = 0; i < this.arr.length; i++) { newVec[i] = this.arr[i]; }
			this.arr = newVec;
		}
		arr[count] = element;
		count++;
	}
	
	public Boolean contains(Class<T> c, T element) {
		Boolean pass = false;
		for(int i = 0; i < this.arr.length; i++) {
			if(this.arr[i] == element) {
				pass = true;
				break;
			}
		}
		return pass;
	}
}
