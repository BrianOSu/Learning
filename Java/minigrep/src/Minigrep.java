import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;
import java.util.ArrayList;
import java.lang.ArrayIndexOutOfBoundsException;

public class Minigrep {
    private String query;
    private String filename;
    private Boolean sensitive = true;

    public Minigrep(String[] args){
        try {
            query = args[0];
        } catch (ArrayIndexOutOfBoundsException e){
            System.out.println("Didn't get a query string");
            e.printStackTrace();
        }

        try {
            filename = args[1];
        } catch (ArrayIndexOutOfBoundsException e){
            System.out.println("Didn't get a filename string");
            e.printStackTrace();
        }

        if(args.length == 3){
            sensitive = false;
            query = query.toLowerCase();
        }
    }

    public void run(){
        ArrayList<String> content = new ArrayList<String>();

        try {
            File myObj = new File(filename);
            Scanner myReader = new Scanner(myObj);
            while (myReader.hasNextLine()) {
                content.add(myReader.nextLine());
            }
            myReader.close();
        } catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }

        String[] res = sensitive ? content.stream().filter(s -> s.contains(query)).toArray(String[]::new)
                                 : content.stream().filter(s -> s.toLowerCase().contains(query)).toArray(String[]::new);

        for(String i: res) {
            System.out.println(i);
        }
    }

}
