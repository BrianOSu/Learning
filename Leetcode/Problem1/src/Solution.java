import java.util.HashMap;
import java.util.Map;

class Solution {
    public int[] twoSum(int[] nums, int target) {
        Map<Integer, Integer> myMap = new HashMap<>();
        for(int i=0; i<nums.length; i++){
            int x = target - nums[i];
            if(myMap.containsKey(x)){
                return new int[] {myMap.get(x), i};
            } else{
                myMap.put(nums[i], i);
            }
        }
        return new int[] {};
    }
}