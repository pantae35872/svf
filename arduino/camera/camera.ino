#include "esp_camera.h"
#include "soc/soc.h" //disable brownout problems
#include "soc/rtc_cntl_reg.h"  //disable brownout problems

#define PWDN_GPIO_NUM     32
#define RESET_GPIO_NUM    -1
#define XCLK_GPIO_NUM      0
#define SIOD_GPIO_NUM     26
#define SIOC_GPIO_NUM     27
  
#define Y9_GPIO_NUM       35
#define Y8_GPIO_NUM       34
#define Y7_GPIO_NUM       39
#define Y6_GPIO_NUM       36
#define Y5_GPIO_NUM       21
#define Y4_GPIO_NUM       19
#define Y3_GPIO_NUM       18
#define Y2_GPIO_NUM        5
#define VSYNC_GPIO_NUM    25
#define HREF_GPIO_NUM     23
#define PCLK_GPIO_NUM     22

void setup() {
  WRITE_PERI_REG(RTC_CNTL_BROWN_OUT_REG, 0);
  // put your setup code here, to run once:
  Serial.begin(115200);
  Serial.setDebugOutput(false);

  camera_config_t config;
  config.ledc_channel = LEDC_CHANNEL_0;
  config.ledc_timer = LEDC_TIMER_0;
  config.pin_d0 = Y2_GPIO_NUM;
  config.pin_d1 = Y3_GPIO_NUM;
  config.pin_d2 = Y4_GPIO_NUM;
  config.pin_d3 = Y5_GPIO_NUM;
  config.pin_d4 = Y6_GPIO_NUM;
  config.pin_d5 = Y7_GPIO_NUM;
  config.pin_d6 = Y8_GPIO_NUM;
  config.pin_d7 = Y9_GPIO_NUM;
  config.pin_xclk = XCLK_GPIO_NUM;
  config.pin_pclk = PCLK_GPIO_NUM;
  config.pin_vsync = VSYNC_GPIO_NUM;
  config.pin_href = HREF_GPIO_NUM;
  config.pin_sscb_sda = SIOD_GPIO_NUM;
  config.pin_sscb_scl = SIOC_GPIO_NUM;
  config.pin_pwdn = PWDN_GPIO_NUM;
  config.pin_reset = RESET_GPIO_NUM;
  config.xclk_freq_hz = 20000000;
  config.pixel_format = PIXFORMAT_JPEG; 
  
  if(psramFound()){
    config.frame_size = FRAMESIZE_UXGA;
    config.jpeg_quality = 10;
    config.fb_count = 2;
  } else {
    config.frame_size = FRAMESIZE_SVGA;
    config.jpeg_quality = 12;
    config.fb_count = 1;
  }

  esp_camera_init(&config);
}

void loop() {
  delay(5000);
  camera_fb_t *fb = esp_camera_fb_get();
  if (fb) {
    uint32_t magic = 0x55AA;
    uint8_t magic_bytes[4];
    magic_bytes[0] = (magic >> 24) & 0xFF;
    magic_bytes[1] = (magic >> 16) & 0xFF;
    magic_bytes[2] = (magic >> 8) & 0xFF;
    magic_bytes[3] = magic & 0xFF;
    Serial.write(magic_bytes, 4);
    size_t jpg_buf_len = 0;
    uint8_t *jpg_buf = NULL;
    frame2jpg(fb, 80, &jpg_buf, &jpg_buf_len);
    uint8_t length_bytes[4];
    length_bytes[0] = (jpg_buf_len >> 24) & 0xFF;
    length_bytes[1] = (jpg_buf_len >> 16) & 0xFF;
    length_bytes[2] = (jpg_buf_len >> 8) & 0xFF;
    length_bytes[3] = jpg_buf_len & 0xFF;
    Serial.write(length_bytes, 4);
    size_t write_pos = 0;
    size_t write_count = 1024;
    while (write_pos < jpg_buf_len) {
      if (jpg_buf_len - write_pos < 1024) {
        write_count = jpg_buf_len - write_pos;
      }
      size_t wrote = Serial.write(jpg_buf+write_pos, write_count);
      write_pos += wrote;
    }
    esp_camera_fb_return(fb);
    free(jpg_buf);
  }
}
