#include <h8/reg3067.h>
#include <mes2.h>
void myWait() {
  int idle1, idle2;
  for (idle1 = 0; idle1 < 2; idle1++) {
    for (idle2 = 0; idle2 < 3800000; idle2++)
      ;
  }
}
int main(void) {
  PBDDR = 0xff;

  while (1) {

    PBDR = 0xfe;
    myWait();
    PBDR = 0xff;
    myWait();
  }

  return (0);
}
