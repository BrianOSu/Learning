class Solution(object):
    def longestPalindromePos(self, s, i, j, lookup):
        print(lookup)
        print("\n")
        if i>j:
            return 0

        if i==j:
            return 1

        key = (i, j)
        if key not in lookup:
            if s[i] == s[j]:
                lookup[key] = self.longestPalindromePos(s, i+1, j-1, lookup) + 2
            else:
                lookup[key] = max(self.longestPalindromePos(s, i, j-1, lookup),
                                  self.longestPalindromePos(s, i+1, j, lookup))
        return lookup[key]

    def longestPalindrome(self, s):
        """
        :type s: str
        :rtype: str
        """
        m = {}
        print(self.longestPalindromePos(s, 0, len(s)-1, m))
        return "bb"
        
if __name__ == '__main__':
    print(f'{Solution().longestPalindrome("babad")}')