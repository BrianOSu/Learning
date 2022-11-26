class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        maxl = 0
        l = len(s)

        for i in range(l):
            d = dict()
            curl = 0
            for (ind, val) in enumerate(s[i:l]):
                if val in d:
                    break
                d[val] = ind
                curl = ind + 1
            maxl = max(maxl, curl)
        return maxl
        
if __name__ == '__main__':
    print(f'{Solution().lengthOfLongestSubstring("abcabcbb")}')
    print(f'{Solution().lengthOfLongestSubstring("bbbbb")}')
    print(f'{Solution().lengthOfLongestSubstring("pwwkew")}')
    print(f'{Solution().lengthOfLongestSubstring(" ")}')
    print(f'{Solution().lengthOfLongestSubstring("")}')