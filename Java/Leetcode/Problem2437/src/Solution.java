import java.util.Arrays;

class Solution {
    public int countTime(String time) {
        // Create char array
        char[] ch = new char[5];
        for(int i=0; i<5; i++){
            ch[i] = time.charAt(i);
        }

        if(ch[0]=='2' && ch[1]==4){
            return 1;
        }

        int cnt = 0;
        cnt += ch[4] == '?' ? 10 : 1;
        cnt *= ch[3] == '?' ? 6 : 1;

        if(ch[0]=='2' || ch[1]==4){
            return 1;
        }


        return cnt;
    }
}