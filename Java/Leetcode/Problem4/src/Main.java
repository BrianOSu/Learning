public class Main {
    public static void main(String[] args) {
        Solution prog = new Solution();
        System.out.println(prog.findMedianSortedArrays(new int[]{1,2}, new int[]{3,4}));
        System.out.println(prog.findMedianSortedArrays(new int[]{1,2}, new int[]{3}));
        System.out.println(prog.findMedianSortedArrays(new int[]{1,2,3,4,5,6,7,8,9}, new int[]{1,2,3}));
    }
}