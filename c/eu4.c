/*
 * =====================================================================================
 *
 *       Filename:  eu4.c
 *
 *    Description:  Euler 4
 *
 *        Version:  1.0
 *        Created:  07/12/12 12:15:56
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  Alex Whitney (), 
 *   Organization:  Imperial College
 *
 * =====================================================================================
 */

#include <stdio.h>

int ispal(int p) { 
    int n = p;
    int i = 0;
    while (n!=0){
        i = 10 * i + n%10;
        n = n/10;
    }
    if (i==p) return 1;
    else return 0;
}

int main(void) {
    int max = 0;
    for(int j = 100;j<1000;j++) {
        for(int k = j;k<1000;k++){
            if(max<j*k && ispal(j*k)) max = j*k;
        }
    }
    printf("Largest palindrome: %d",max);
    return 0;
}

