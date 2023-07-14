#include <iostream>
#include <stdio.h>
//#include "myheader.h"

extern "C" int add(int a, int b){
    int y = static_cast<int>(b);
    return a + y;
}

int main(){

    int x = add(5, 10);
    printf("Number %d\n", x);

}

