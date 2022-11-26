public class Main {
    public static void main(String[] args) {
        Solution prog = new Solution();
        System.out.println(prog.countTime("?5:00"));
        System.out.println(prog.countTime("15:10"));
        System.out.println(prog.countTime("15:1?"));
        System.out.println(prog.countTime("15:?0"));
        System.out.println(prog.countTime("24:??"));
        System.out.println(prog.countTime("2?:10"));
        System.out.println(prog.countTime("?4:10"));
    }
}