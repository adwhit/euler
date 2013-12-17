/*
 * =====================================================================================
 *
 *       Filename:  eu3.c
 *
 *    Description:  Euler 3
 *
 *        Version:  1.0
 *        Created:  07/12/12 11:14:10
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  Alex Whitney (), 
 *   Organization:  Imperial College
 *
 * =====================================================================================
 */
#include <stdio.h>

long prime_fac(long n){
    for (long i=2;i<=n/2;i++) {
        if (n%i == 0) {
            printf("n:%lu,i:%lu\n",n,i);
            return prime_fac(n/i);
        }
    }
    return n;
}

int main(void) {
    long pf = 6008514751434;
    //long pf = 345;
    printf("Largest prime factor of %lu is %lu",pf,prime_fac(pf));
    return 0;
}


