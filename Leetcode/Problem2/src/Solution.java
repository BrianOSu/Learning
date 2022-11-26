class Solution {

    public class ListNode {
        int val;
        ListNode next;

        ListNode() {}
        ListNode(int val) { this.val = val; }
        ListNode(int val, ListNode next) { this.val = val; this.next = next; }
    }

    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        ListNode l = new ListNode(0);
        ListNode m = l;

        int carry = 0;

        while(l1 != null || l2 != null){
            int sum = carry;

            if(l1 != null){
                sum += l1.val;
                l1 = l1.next;
            }

            if(l2 != null){
                sum += l2.val;
                l2 = l2.next;
            }

            if(sum>9){
                sum -= 10;
                carry = 1;
            } else {
                carry = 0;
            }

            ListNode n = new ListNode(sum);
            m.next = n;
            m = m.next;

        };

        if(carry == 1){
            ListNode n = new ListNode(1);
            m.next = n;
        }
        return l.next;
    }
}