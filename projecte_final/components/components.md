- GPS: [ZED-F9P-15B](https://www.u-blox.com/en/product/zed-f9p-module)
  - [Data sheet](https://content.u-blox.com/sites/default/files/documents/ZED-F9P-15B_DataSheet_UBX-23009090.pdf)
  - Price: [98.83€/uni](https://www.u-blox.com/en/product/zed-f9p-module)
  - Pins: 2 (UART)
  - Power:
    - VCC: 3.3V
    - Current Peak: 120mA
    - Current: 70mA
  - Update rate: 24Hz
  - Precision: 0.01m (Vertical and Horizontal)

- Temperatura i humitat: [SHT35-DIS](https://mou.sr/3Whj1kr)
  - [Data sheet](https://www.mouser.es/datasheet/2/682/Sensirion_Humidity_Sensors_SHT3x_Datasheet_digital-2001026.pdf)
  - Price: [5.42€/uni](https://mou.sr/3Whj1kr)
  - Pins: 2 (I2C: SCL i SDA)
  - Power:
    - VCC: 2.15V - 5.5V
    - Current: 600µA
  - Update rate: 10Hz
  - Precision: ±0.2°C
  
- Orientació: [BNO055](https://www.bosch-sensortec.com/products/smart-sensor-systems/bno055/)
  - [Data sheet](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bno055-ds000.pdf)
  - Price: [8.65€/uni](https://mou.sr/4gNEZnv)
  - Pins: 4 - 2 (I2C: SCL i SDA) + 2 (GPIO: PC_ADDR_SEL, INT)
    - Input: Can be connected to 32KHZ external clock
  - Power:
    - VCC: 2.4V - 3.6V
    - Current: 
      - Mesuring: 12.3 mA
      - Suspend mode: 0.04mA
  - Update rate: 100Hz
  - Precision: ±2.5 degrees

- Altitud i pressió: [BMP390](https://www.bosch-sensortec.com/products/environmental-sensors/pressure-sensors/bmp390/)
  - [Data sheet](https://www.bosch-sensortec.com/media/boschsensortec/downloads/product_flyer/bst-bmp390-fl000.pdf)
  - Price: [2.08€/uni](https://www.mouser.de/ProductDetail/Bosch-Sensortec/BMP390?qs=QNEnbhJQKvYQVfvRMgo2YA%3D%3D)
  - Pins: 2 (I2C: SCL i SDA)
  - Power:
    - VCC: 1.65V - 3.6V
    - Current: 3.2µA at 1Hz
  - Update rate: 1HZ
  - Precision: ±0.50 hPa

- RTC: [DS3231MZ/V+](https://www.digikey.es/es/products/detail/analog-devices-inc-maxim-integrated/DS3231MZ-V/3133910)
  - [Data sheet](https://www.analog.com/media/en/technical-documentation/data-sheets/DS3231M.pdf)
  - Price: [6.35€/uni](https://www.digikey.es/es/products/detail/analog-devices-inc-maxim-integrated/DS3231MZ-V/3133910)
  - Pins: 3 - 2 (I2C: SCL, SDA) + 1 (GPIO: INTERRUPT)
    - OUTPUT CLK 32KHZ (for external clock)
  - Power:
    - VCC: 2.3V - 5.5V
    - Current: 130µA
  - Update rate: 1HZ (Interrupt each second)
  - Precision: ±0.432 Second/Day

- Comunicacions: [SX1278](https://www.semtech.com/products/wireless-rf/lora-connect/sx1278)
  - [Data sheet](https://semtech.my.salesforce.com/sfc/p/#E0000000JelG/a/2R0000001Rc1/QnUuV9TviODKUgt_rpBlPz.EZA_PNK7Rpi8HA5..Sbo)
  - Price: [5.53€/uni](https://mou.sr/3BSIcTD)
  - Pins: 9 - 4 (SPI: SCK, MISO, MOSI, NSS) + 5 (GPIO: DIO0-DIO5)
  - Power:
    - VCC: 1.8V - 3.7V
    - Current:
      - Idle: 1.5µA
      - Receive: 12µA
      - Transmit: 20mA - 120mA
  - Distance: 200m - 820m

- Radiació: https://www.electan.com/pocket-p-10168.html


| Component         | Type                     | Data Sheet                                                                                                                     | Price                                                                                                         | Pins                                            | VCC          | Current                                            | Update Rate | Precision                       |
| ----------------- | ------------------------ | ------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------- | ----------------------------------------------- | ------------ | -------------------------------------------------- | ----------- | ------------------------------- |
| **ZED-F9P-15B**   | GPS                      | [Data sheet](https://content.u-blox.com/sites/default/files/documents/ZED-F9P-15B_DataSheet_UBX-23009090.pdf)                  | [98.83€/uni](https://www.u-blox.com/en/product/zed-f9p-module)                                                | 2 (UART)                                        | 3.3V         | Peak: 120mA, Typical: 70mA                         | 24Hz        | 0.01m (Vertical and Horizontal) |
| **SHT35-DIS**     | Temperature and Humidity | [Data sheet](https://www.mouser.es/datasheet/2/682/Sensirion_Humidity_Sensors_SHT3x_Datasheet_digital-2001026.pdf)             | [5.42€/uni](https://mou.sr/3Whj1kr)                                                                           | 2 (I2C: SCL, SDA)                               | 2.15V - 5.5V | 600µA                                              | 10Hz        | ±0.2°C                          |
| **BNO055**        | Orientation              | [Data sheet](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bno055-ds000.pdf)                   | [8.65€/uni](https://mou.sr/4gNEZnv)                                                                           | 4 (I2C: SCL, SDA + GPIO: PC_ADDR_SEL, INT)      | 2.4V - 3.6V  | Measuring: 12.3 mA, Suspend: 0.04mA                | 100Hz       | ±2.5 degrees                    |
| **BMP390**        | Altitude and Pressure    | [Data sheet](https://www.bosch-sensortec.com/media/boschsensortec/downloads/product_flyer/bst-bmp390-fl000.pdf)                | [2.08€/uni](https://www.mouser.de/ProductDetail/Bosch-Sensortec/BMP390?qs=QNEnbhJQKvYQVfvRMgo2YA%3D%3D)       | 2 (I2C: SCL, SDA)                               | 1.65V - 3.6V | 3.2µA at 1Hz                                       | 1Hz         | ±0.50 hPa                       |
| **DS3231MZ/V+**   | RTC                      | [Data sheet](https://www.analog.com/media/en/technical-documentation/data-sheets/DS3231M.pdf)                                  | [6.35€/uni](https://www.digikey.es/es/products/detail/analog-devices-inc-maxim-integrated/DS3231MZ-V/3133910) | 3 (I2C: SCL, SDA + GPIO: INTERRUPT)             | 2.3V - 5.5V  | 130µA                                              | 1Hz         | ±0.432 Second/Day               |
| **SX1278**        | Communications           | [Data sheet](https://semtech.my.salesforce.com/sfc/p/#E0000000JelG/a/2R0000001Rc1/QnUuV9TviODKUgt_rpBlPz.EZA_PNK7Rpi8HA5..Sbo) | [5.53€/uni](https://mou.sr/3BSIcTD)                                                                           | 9 (SPI: SCK, MISO, MOSI, NSS + GPIO: DIO0-DIO5) | 1.8V - 3.7V  | Idle: 1.5µA, Receive: 12µA, Transmit: 20mA - 120mA | -           | Distance: 200m - 820m           |
| **Pocket Geiger** | Radiation                |                                                                                                                                |                                                                                                               |                                                 |              |                                                    |             |                                 |