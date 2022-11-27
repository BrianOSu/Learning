import java.util.ArrayList;

class Solution {
    // O(m+n/2) Solution
    public double findMedianSortedArrays(int[] nums1, int[] nums2) {
        int n = nums1.length, m=nums2.length;
        int l=0, r=0, ans=0, prev=0;

        for(int i = 0; i<=(m+n)/2; i++){
            prev = ans;
            if(l == n)
                ans = nums2[r++];
            else if(r == m)
                ans = nums1[l++];
            else if(nums1[l] < nums2[r])
                ans = nums1[l++];
            else
                ans = nums2[r++];
        }

        return (m+n) % 2 == 0 ? (prev+ans)/2f : ans;
    }
}