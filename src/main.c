#include <stdio.h>
#include "config.h"

int main(int argc, char** argv) {
  printf("just trying to undrestand cmake");
  printf("\n");
  printf("Program name: %s", PROG_NAME);
  printf("\n");
  printf("Full program name: %s", PROG_FULLNAME);
  return 0;
}
