/*
 * =====================================================================================
 *
 *       Filename:  eu1.c
 *
 *    Description:  Euler 1
 *
 *        Version:  1.0
 *        Created:  07/12/12 10:37:56
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  Alex Whitney (), 
 *   Organization:  Imperial College
 *
 * =====================================================================================
 */

#include <stdio.h>

int main(void) {
    int s = 0;
    for (int i;i < 1000;i++) {
        if (i%5 == 0 || i%3 == 0) 
            s += i;
    }
    printf("Your sum is %d",s);
    return 0;
}
