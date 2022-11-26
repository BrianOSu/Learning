from typing import List

class Solution():
    def __init__(self):
        self.Dict = dict()

    def twoSum(self, nums: List[int], target: int) -> List[int]:
        for i,v in enumerate(nums):
            if v in self.Dict:
                return [self.Dict[v], i]
            else:
                self.Dict[target - v] = i
        return []

if __name__ == '__main__':
    print(f"{Solution().twoSum([2, 7, 11, 15], 9)}")
    print(f"{Solution().twoSum([3, 2, 4], 6)}")
    print(f"{Solution().twoSum([3, 3], 6)}")