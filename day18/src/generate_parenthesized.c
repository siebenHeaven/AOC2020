#include <stdio.h>
#include <string.h>
/* Fortran 1 compiler inspired, from wikipedia entry:
 * https://en.wikipedia.org/wiki/Operator-precedence_parser#Alternative_methods
 **/
int main(int argc, char *argv[]) {
  int i;
  printf("((((");
  for (i=1;i!=argc;i++) {
    if (argv[i] && !argv[i][1]) {
      switch (*argv[i]) {
          case '(': printf("(((("); continue;
          case ')': printf("))))"); continue;
          case '^': printf(")^("); continue;
          case '+': printf("))+(("); continue;
          case '/': printf("))/(("); continue;
          case '*':
            if (i == 1 || strchr("(^+/*-", *argv[i-1]))
              printf("*");
            else
              printf(")))*(((");
            continue;
          case '-':
            if (i == 1 || strchr("(^+/*-", *argv[i-1]))
              printf("-");
            else
              printf(")))-(((");
            continue;
      }
    }
    printf("%s", argv[i]);
  }
  printf("))))\n");
  return 0;
}
