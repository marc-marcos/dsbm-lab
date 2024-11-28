#include "driver.c"
#include<wiringPi.h>

int main() {
  Config_Pins();
  SPI_TFT_Reset();

  int fd = open_port();

  fill(0xFFFFFF);

  while(1) {
    write_stop_and_go(fd);

    pos_t pos = read_port(fd);

    paint_square(pos.x, pos.y, 3);
  }
}
