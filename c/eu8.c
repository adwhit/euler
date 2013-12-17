/*
 * =====================================================================================
 *
 *       Filename:  eu8.c
 *
 *    Description:  Euler 8
 *
 *        Version:  1.0
 *        Created:  12/10/2012 09:57:50 AM
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <stdio.h>
#include <stdlib.h>

int main(void){
    int i;
    int ;
    char nums[1001];
    FILE *f = fopen("nums.txt", "rt");
    fgets(nums,sizeof(nums),f);
    puts(nums);
    for(i = 0;i<10;i++){
        n = (int)(nums[i]-'0');
        printf("%c,%d\n",nums[i],(int)(nums[i]-'0'));
    }
    return 0;
}
