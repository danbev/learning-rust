#include "cfunctions.h"
#include "stdio.h"
#include <stdlib.h>

void doit(int nr) {
  printf("Do something. nr: %d\n", nr);
  printf("Going to call exit\n");
  exit(1);
  printf("After calling exit\n");
}

void print_string(char* s) {
  printf("print_string. s: %s\n", s);
}
