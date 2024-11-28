#include "driver.c"
#include<wiringPi.h>

int main() {
  Config_Pins();
  SPI_TFT_Reset();

  fill(0xFFFFFF);
  for (int i; i < 10; ++i) {
    paint_string("Hello world!", 100 + 8*i, 100 + 2*i);
  }

  for (int i; i < 10; ++i) {
    paint_rainbow_string("Hello world!", 20 + 8*i, 20 + 2*i);
  }
}
