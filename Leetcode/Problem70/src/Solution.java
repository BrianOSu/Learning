public class Solution {
    public int climbStairs(int n) {
        int nextStep = 0, first = 1, second = 1;
        for(int i=0; i<n; i++){
            nextStep = first + second;
            first = second;
            second = nextStep;
        }
        return(first);
    }
}
