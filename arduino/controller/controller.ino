#include <WiFi.h>
#include <ArduinoBLE.h>
#include <EEPROM.h>
#include <ArduinoJson.h>
#include <Streaming.h>

struct SaveData {
  bool wifi;
  char wifi_ssid[32];
  char wifi_password[64];
  bool device;
  char device_id[64];
};

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
  while (!Serial) {
    delay(1);
  }
  EEPROM.begin();  // Initialize EEPROM with 512 bytes
  BLE.setDeviceName("BerryBotics");
  BLE.setLocalName("BerryBotics");

  if (!BLE.begin()) {
    Serial.println("- Starting Bluetooth® Low Energy module failed!");
    while (1);
  }

  BLE.setAdvertisedService(berryService);
  berryService.addCharacteristic(berryRequestCharacteristic);
  berryService.addCharacteristic(berryResponseCharacteristic);
  BLE.addService(berryService);
  BLE.advertise();
  memset(&data, 0, sizeof(SaveData));
  EEPROM.get(0, data);
  Serial << "Device: " << data.device << endl;
  Serial << "Wifi: " << data.wifi << endl;
  Serial << "SSID: " << String(data.wifi_ssid) << endl;
  Serial << "PASSWORD: " << String(data.wifi_password) << endl;
  Serial << "DEVICE_ID: " << String(data.device_id) << endl;
}

void loop() {
  BLEDevice central = BLE.central();
  if (central) {
    while (central.connected()) {
      if (berryRequestCharacteristic.written()) {
        String incoming = berryRequestCharacteristic.value();
        JsonDocument doc;
        deserializeJson(doc, incoming);
        Serial.println("Incoming Request");
        Serial.println(incoming);
        if (doc["request"].is<String>()) {
          String request = doc["request"];
          if (request.equals("wifi")) {
            JsonDocument doc;
            if (data.wifi == false) {
              doc["need_wifi"] = true;
            } else {
              doc["need_wifi"] = false;
            }
            String data;
            serializeJson(doc, data);
            berryResponseCharacteristic.setValue(data);
          } else if (request.equals("wifi-set")) {
            String ssid = doc["ssid"];
            String password = doc["password"];
            data.wifi = true;
            strncpy((char*)&data.wifi_ssid, ssid.c_str(), sizeof(data.wifi_ssid));
            strncpy((char*)&data.wifi_password, password.c_str(), sizeof(data.wifi_password));
            EEPROM.put(0, data);
          } else if (request.equals("device-id")) {
            JsonDocument doc;
            doc["device_id"] = String(data.device_id);
            String data;
            serializeJson(doc, data);
            berryResponseCharacteristic.setValue(data);
          } else if (request.equals("close")) {
            central.disconnect();
          }
        }
      }
    }
  }

  if (WiFi.status() == WL_CONNECTED) {
    if (!client.connected()) {
      if (client.connect("192.168.1.123", 4000)) {
        Serial << "Connection Successful" << endl;
      }
    } else {
      if (client.available()) {
        uint8_t header[16];
        client.read(header, 16);
      }
    }
  } else if (wifi_timeout < 3 && data.wifi) {
    Serial << "Connecting to wifi. retry count: " << wifi_timeout << endl;
    WiFi.begin((const char*)&data.wifi_ssid, (const char*)&data.wifi_password);
    delay(1000);
    wifi_timeout++;
  } else if (wifi_timeout >= 3 && data.wifi) {
    data.wifi = false;
    wifi_timeout = 0;
    EEPROM.put(0, data);
  }
  // if (WiFi.status() == WL_CONNECTED) {
  //   if (!client.connected()) {
  //     client.connect("35.198.240.174", 4000);
  //     return;
  //   }
  // } else if (wifi_timeout < 10 && data.wifi) {
  //   WiFi.begin((const char*)&data.wifi_password, (const char*)&data.wifi_password);
  //   wifi_timeout++;
  // } else {
  //   data.wifi = false;
  //   wifi_timeout = 0;
  //   EEPROM.put(0, data);
  // }
  // delay(100);
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
