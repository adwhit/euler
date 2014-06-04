import java.util.Arrays;
import java.math.BigInteger;

class Six {

    public static void main(String []args) {
        Six six = new Six();
        int[] arr = six.genArray(0,100);
        int sumofsqr = six.sum(six.squareEach(arr));
        int sqrofsum = six.square(six.sum(arr));
        System.out.println(sqrofsum - sumofsqr);
    }

    int square(int x) {
        return x*x;
    }

    int sum(int[] xs) {
        int s = 0;
        for (int i: xs) {
            s += i;
        }
        return s;
    }

    int[] squareEach(int[] xs) {
        int[] arr = new int[xs.length];
        for (int i=0;i<xs.length;i++) {
            arr[i] = this.square(xs[i]);
        }
        return arr;
    }

    int[] genArray(int start, int stop) {
        int[] arr = new int[stop-start];
        for (int i=start;i<stop;i++) {
            arr[i-start] = i;
        }
        return arr;
    }

}
