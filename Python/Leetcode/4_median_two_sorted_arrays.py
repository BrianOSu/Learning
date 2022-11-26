from typing import List
from statistics import median

class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        ans: List[int] = []
        i = 0
        j = 0

        while i<len(nums1) or j<len(nums2):
            if i==len(nums1):
                ans.extend(nums2[j:len(nums2)])
                j = len(nums2)
            elif j==len(nums2):
                ans.extend(nums1[i:len(nums1)])
                i = len(nums1)
            elif nums1[i]<nums2[j]:
                ans.append(nums1[i])
                i += 1
            else:
                ans.append(nums2[j])
                j+=1
                
        return median(ans)

if __name__ == '__main__':
    print(f'{Solution().findMedianSortedArrays([1,3], [2])}')