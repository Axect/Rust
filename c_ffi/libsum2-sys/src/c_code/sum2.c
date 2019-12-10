#include <stdio.h>

extern void sum2_(int *x, int *y);

int sum2(int x) {
    int y = 0;
    sum2_(&x, &y);
    return y;
}