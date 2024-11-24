#include <WiFi.h>

const char* ssid = "TINN_2.4G";
const char* password = "0635415591";

WiFiClient client;

void setup() {
  // put your setup code here, to run once:
  Serial1.begin(57600);

  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    delay(1000);
    Serial.print(".");
  }
}

void loop() {
  uint8_t magic_bytes[4];
  Serial1.readBytes(magic_bytes, 4);
  uint32_t magic = (uint32_t)magic_bytes[0] << 24 |
                 (uint32_t)magic_bytes[1] << 16 |
                 (uint32_t)magic_bytes[2] << 8 |
                 (uint32_t)magic_bytes[3];;
  if (magic != 0x55AA) {
    return;
  }
  uint8_t length_byte[4];
  size_t readed = Serial1.readBytes(length_byte, 4);
  while (readed != 4) {
    readed = Serial1.readBytes(length_byte, 4 - readed);
  }
  uint32_t length = (uint32_t)length_byte[0] << 24 |
                 (uint32_t)length_byte[1] << 16 |
                 (uint32_t)length_byte[2] << 8 |
                 (uint32_t)length_byte[3];
  size_t wrotes = 0;
  size_t write_count = 128;
  if (readed > 0 && length > 0 && client.connect("192.168.1.123", 4000)) {
    client.write(length_byte, 4);
    while (wrotes < length) {
      if (length - wrotes < 128) {
        write_count = length - wrotes;
      } 
      uint8_t *buffer = (uint8_t*)calloc(0, write_count);
      size_t readed = Serial1.readBytes(buffer, write_count);
      size_t wrote = client.write(buffer, readed);
      wrotes += wrote;
      free(buffer);
    }
    client.stop();
  }
}
