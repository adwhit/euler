/*
 * =====================================================================================
 *
 *       Filename:  eu10.c
 *
 *    Description:  PrimeGen
 *
 *        Version:  1.0
 *        Created:  07/12/12 14:27:40
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  Alex Whitney (), 
 *   Organization:  Imperial College
 *
 * =====================================================================================
 */

#define NPRIMES 3000000LL
#include <stdio.h>


int sieve(long long n){
    long long sum = 0;
    long long i,j;
    int primebool[n];
    for(long long a=2;a<n;a++) primebool[a]=1;

    //generate bool array of primes to n
    for (i=2; i<n; i++){
        if (primebool[i]){
            sum += i;
            for(j = i*2;j<n;j+=i){
                primebool[j] = 0;
            }
        }
    }

    printf("Sum of primes up to %llu is %llu\n",n,sum);
    return 1 ;
}

int main(void){
    sieve(NPRIMES);
    printf("Int %lu \nLong %lu",sizeof(int),sizeof(long));
    return 0;
}
