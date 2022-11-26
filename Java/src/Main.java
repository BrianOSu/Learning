public class Main{
    
    public static void main(String[] args) {
        System.out.println("Testing question regex logic");
        System.out.println(Regex.question("testing","tes?ing"));
        System.out.println(Regex.question("testing","tes?in"));
        System.out.println(Regex.question("testing","ter?ing"));

        System.out.println("Testing star regex logic");
        System.out.println(Regex.star("testing","*t?s*ng*"));
        System.out.println(Regex.star("thisisntworking","*t?s*ng*"));
        System.out.println(Regex.star("thisisntworking","t??s*ng"));
        System.out.println(Regex.star("thisisntworkingr","t??s*ng")); // Expect false
    }

}