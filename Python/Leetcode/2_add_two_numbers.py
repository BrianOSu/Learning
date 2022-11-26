from typing import List
from typing import Optional

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution():
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        return self.add(l1, l2, 0)

    def add(self, l1: Optional[ListNode], l2: Optional[ListNode], carry: int) -> Optional[ListNode]:
        sum = carry
        if(l1 is not None):
            sum += l1.val
            l1 = l1.next

        if(l2 is not None):
            sum += l2.val
            l2 = l2.next

        ans = ListNode(sum%10)

        if(l1 is not None or l2 is not None or sum>9):
            ans.next = self.add(l1, l2, int(sum/10))
            
        return ans

if __name__ == '__main__':
    l1 = ListNode(2, ListNode(4, ListNode(3)))
    l2 = ListNode(5, ListNode(6, ListNode(4)))
    ans = Solution().addTwoNumbers(l1, l2)
    while(ans is not None):
        print(f"{ans.val}")
        ans = ans.next