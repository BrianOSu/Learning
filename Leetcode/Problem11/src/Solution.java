class Solution {
    public int maxArea(int[] height) {
        int l=0, r=height.length-1, max=0;

        while(l<r){
            int vol = Math.min(height[l], height[r]) * (r-l);
            if(vol > max)
                max = vol;
            if(height[l] > height[r])
                r -= 1;
            else
                l +=1;
        }
        return max;
    }
}