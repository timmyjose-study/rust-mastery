#include <stdio.h>

extern struct Point *new_point(int, int);
extern void inspect_point(struct Point*);

int main() {
  struct Point *p = new_point(100, 200);
  inspect_point(p);

  return 0;
}
