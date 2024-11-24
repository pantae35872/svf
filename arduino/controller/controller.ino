#include <WiFi.h>
#include <ArduinoBLE.h>
#include <EEPROM.h>
#include <ArduinoJson.h>

struct SaveData {
  bool wifi;
  char wifi_ssid[32];
  char wifi_password[64];
  bool device;
  char device_id[64];
};

const char* ssid = "TINN_2.4G";
const char* password = "0635415591";
const char* deviceServiceUuid = "f901b2a6-02a1-40ab-8b44-6471bd5886af";
const char* deviceServiceRequestCharacteristicUuid = "9fbdeb54-dab7-42a0-bca1-6f6a80240c45";
const char* deviceServiceResponseCharacteristicUuid = "3457d261-fdf2-4e43-98f6-6f9064ff8abd";

BLEService berryService(deviceServiceUuid);
BLEStringCharacteristic berryRequestCharacteristic(deviceServiceRequestCharacteristicUuid, BLEWrite, 128);
BLEStringCharacteristic berryResponseCharacteristic(deviceServiceResponseCharacteristicUuid, BLENotify, 128);
WiFiClient client;

unsigned long previousMillis = 0;   // Tracks the last time the event occurred
const unsigned long interval = 10;  // 5 seconds (5000 milliseconds)
int wifi_timeout = 0;

SaveData data;

void setup() {
  // put your setup code here, to run once:
  Serial1.begin(57600);
  Serial.begin(9600);
  EEPROM.begin();  // Initialize EEPROM with 512 bytes
  BLE.setDeviceName("BerryBotics");
  BLE.setLocalName("BerryBotics");

  if (!BLE.begin()) {
    Serial.println("- Starting Bluetooth® Low Energy module failed!");
    while (1)
      ;
  }

  BLE.setAdvertisedService(berryService);
  berryService.addCharacteristic(berryRequestCharacteristic);
  berryService.addCharacteristic(berryResponseCharacteristic);
  BLE.addService(berryService);
  BLE.advertise();
  // WiFi.begin(ssid, password);
  // while (WiFi.status() != WL_CONNECTED) {
  //   delay(1000);
  //   Serial.print(".");
  // }
  memset(&data, 0, sizeof(SaveData));
  EEPROM.get(0, data);
}

void loop() {
  BLEDevice central = BLE.central();
  if (central) {
    bool wifi_sent = false;
    bool device_sent = false;
    while (central.connected()) {
      if (data.wifi == false && !wifi_sent) {
        JsonDocument doc;
        doc["wifi_request"] = "";
        String value;
        serializeJson(doc, value);
        berryResponseCharacteristic.setValue(value);
        wifi_sent = true;
      }
      if (data.device == true && !device_sent) {
        JsonDocument doc;
        doc["device_id"] = "111";
        String value;
        serializeJson(doc, value);
        berryResponseCharacteristic.setValue(value);
        device_sent = true;
      }
      if (berryRequestCharacteristic.written()) {
        String wifi = berryRequestCharacteristic.value();
        JsonDocument doc;
        deserializeJson(doc, wifi);
        if (doc["ssid"].is<String>() && doc["password"].is<String>()) {
          String ssid = doc["ssid"];
          String password = doc["password"];
          data.wifi = true;
          EEPROM.put(0, data);
        } else if (doc["close"].is<String>()) {
          central.disconnect();
        }
      }
      delay(10);
    }
  }

  if (WiFi.status() == WL_CONNECTED) {
    if (!client.connected()) {
      client.connect("35.198.240.174", 4000);
      return;
    }
  } else if (wifi_timeout < 10 && data.wifi) {
    WiFi.begin((const char*)&data.wifi_password, (const char*)&data.wifi_password);
    wifi_timeout++;
  } else {
    data.wifi = false;
    wifi_timeout = 0;
    EEPROM.put(0, data);
  }
  delay(100);
  // Do other tasks here
  //delay(100); // Simulating work (replace with your actual task)

  // uint8_t magic_bytes[4];
  // Serial1.readBytes(magic_bytes, 4);
  // uint32_t magic = (uint32_t)magic_bytes[0] << 24 |
  //                (uint32_t)magic_bytes[1] << 16 |
  //                (uint32_t)magic_bytes[2] << 8 |
  //                (uint32_t)magic_bytes[3];;
  // if (magic != 0x55AA) {
  //   return;
  // }
  // uint8_t length_byte[4];
  // size_t readed = Serial1.readBytes(length_byte, 4);
  // while (readed != 4) {
  //   readed = Serial1.readBytes(length_byte, 4 - readed);
  // }
  // uint32_t length = (uint32_t)length_byte[0] << 24 |
  //                (uint32_t)length_byte[1] << 16 |
  //                (uint32_t)length_byte[2] << 8 |
  //                (uint32_t)length_byte[3];
  // size_t wrotes = 0;
  // size_t write_count = 128;
  // if (readed > 0 && length > 0 && client.connect("192.168.1.123", 4000)) {
  //   client.write(length_byte, 4);
  //   while (wrotes < length) {
  //     if (length - wrotes < 128) {
  //       write_count = length - wrotes;
  //     }
  //     uint8_t *buffer = (uint8_t*)calloc(0, write_count);
  //     size_t readed = Serial1.readBytes(buffer, write_count);
  //     size_t wrote = client.write(buffer, readed);
  //     wrotes += wrote;
  //     free(buffer);
  //   }
  //   client.stop();
  // }
}
