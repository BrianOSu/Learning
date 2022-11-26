import java.util.HashMap;
public class Solution {
    public int lengthOfLongestSubstring(String s) {
        int max = 0;
        int l = s.length();

        for(int i=0; i<l; i++){
            HashMap<Character, Integer> map = new HashMap<Character, Integer>();
            for(int n=i; n<l; n++){
                if(map.containsKey(s.charAt(n)))
                    break;
                else
                    map.put(s.charAt(n), n);
            }
            max = Math.max(map.size(), max);
        }

        return max;
    }
}
