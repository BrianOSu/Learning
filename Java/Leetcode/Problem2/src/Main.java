public class Main {
    public static void main(String[] args) {
        Solution prog = new Solution();
        Solution.ListNode l1 = prog.new ListNode(2, prog.new ListNode(4, prog.new ListNode(3)));
        Solution.ListNode l2 = prog.new ListNode(5, prog.new ListNode(6, prog.new ListNode(4)));

        Solution.ListNode l =prog.addTwoNumbers(l1, l2);
        while(l != null){
            System.out.println(l.val);
            l = l.next;
        }
    }
}