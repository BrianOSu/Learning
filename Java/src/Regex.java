public class Regex {
    static boolean question(String in, String compare){
        int l = in.length();
        int r = compare.length();
        if( l != r)
            return false;

        for(int i=0; i<l; i++){
            if(in.charAt(i) != compare.charAt(i) && compare.charAt(i) != '?')
                return false;
        }
        return true;
    }

    static boolean star(String in, String compare){
        // Break up comparison on stars
        String[] parts = compare.split("\\*");
        int pos=0;
        for(String i: parts){
            if(i == "")
                continue;

            // Loop adjusting where the left pos starts at
            for(int l = pos; l <= in.length() - i.length(); l++){
                // find right pos
                int r = l + i.length();
                if(question(in.substring(l, r), i)){
                    pos = r;
                    break;
                }
                if(r == in.length())
                    return false;
            }
        }
        return true;
    }
}
