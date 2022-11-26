public class Solution {
    public boolean isPalindrome(int x) {
        String test = Integer.toString(x);
        int l = test.length();
        for(int i = 0; i<l/2; i++){
            if(test.charAt(i) != test.charAt(l-i-1))
                return false;
        }
        return true;
    }
}
