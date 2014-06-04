import java.math.*;

class FortyThree {

    private static String digits = "0123456789";
    private static int[] primes = {2,3,5,7,11,13,17};
    private BigInteger total;

    public static void main(String[] args) {
        FortyThree ft = new FortyThree();
        ft.permutation("", FortyThree.digits);
        System.out.println(ft.total);
    }

    public FortyThree() {
        this.total = new BigInteger("0");
    }

    private void permutation(String prefix, String str) {
        int n = str.length();
        if (n==0) {
            for (int i=0;i<7;i++) {
                int slice = extractInt(prefix, i+1);
                int denom = FortyThree.primes[i];
                if(!(slice % denom == 0)) {
                    return;
                }
            }
            BigInteger big = new BigInteger(prefix);
            this.total = this.total.add(big);
            return;
        }
        else {
            for (int i=0; i<n;i++) {
                 permutation(prefix + str.charAt(i), str.substring(0,i) + str.substring(i+1, n));
            }
            return;
        }
    }

    private int extractInt(String str, int startIndex) {
        return Integer.parseInt(str.substring(startIndex, startIndex+3));
    }

}
