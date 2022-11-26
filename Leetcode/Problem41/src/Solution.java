import java.util.HashMap;

class Solution {
    public int firstMissingPositive(int[] nums) {
        HashMap<Integer, Boolean> map = new HashMap<Integer, Boolean>();
        for(int i=0; i<nums.length; i++){
            map.put(nums[i], true);
        }

        for(int i=1; i<=nums.length; i++){
            if(null == map.get(i)){
                return i;
            }
        }

        return nums.length+1;
    }
}