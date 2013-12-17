/*
 * =====================================================================================
 *
 *       Filename:  eu5.c
 *
 *    Description:  Euler 5
 *
 *        Version:  1.0
 *        Created:  07/12/12 12:33:25
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  Alex Whitney (), 
 *   Organization:  Imperial College
 *
 * =====================================================================================
 */

#include <stdio.h>


long gcd(long n,long m){
    if (m>n) {
        long tmp = m;
        m = n;
        n = tmp;
    }
    while(1){
        long rem = n%m;
        if(rem == 0) return m;
        n = m;
        m = rem;
    }
}

long lcm(long n, long m){
    return m*n/gcd(n,m);
}

int main(void){
    long res = 1;
    for (long i=2;i<=20;i++){
        res = lcm(res,i);
        printf("%lu %lu\n",res,i);
    }
    printf("Result: %lu\n",res);
    return 0;
}
