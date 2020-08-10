#include <stdio.h>
#include "lib.h"

int main(void) {
  int before = 3;
  int after = inc(before);
  printf("before %d, after %d\n", before, after);
  return 0;
}
