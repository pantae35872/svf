#include <WiFi.h>
#include <ArduinoBLE.h>
#include <EEPROM.h>
#include <ArduinoJson.h>
#include <Streaming.h>

#define RESET_PIN 4

enum class ClientPacket : uint32_t {
  ReportId = 0,
  ReportSensors,
  ImageFrame,
};

enum class ServerPacket : uint32_t {
  UpdateCooler = 0,
  WaterPulse,
  ResponseId,
};


enum class NetworkStatus {
  WiFiError,
  ServerError,
  NoWiFI,
  Ok,
};

class BufferWriter {
private:
  uint8_t* buffer;
  size_t capacity;
  size_t writePos;

  void ensureCapacity(size_t additionalSize) {
    if (writePos + additionalSize > capacity) {
      size_t newCapacity = capacity + additionalSize + 64;
      buffer = (uint8_t*)realloc(buffer, newCapacity);
      capacity = newCapacity;
    }
  }

public:
  BufferWriter(size_t initialCapacity = 64)
    : buffer((uint8_t*)malloc(initialCapacity)), capacity(initialCapacity), writePos(0) {}

  ~BufferWriter() {
    free(buffer);
  }

  BufferWriter& writeBytes(const uint8_t* data, size_t length) {
    ensureCapacity(length);
    memcpy(buffer + writePos, data, length);
    writePos += length;
    return *this;
  }

  BufferWriter& writeString(const char* str) {
    size_t length = strlen(str);

    if (length > UINT32_MAX) {
      length = UINT32_MAX;
    }

    writeU32(static_cast<uint32_t>(length));

    return writeBytes(reinterpret_cast<const uint8_t*>(str), length);
  }

  // Writes a 64-bit integer
  BufferWriter& writeI64(int64_t data) {
    return writeBytes(reinterpret_cast<const uint8_t*>(&data), sizeof(data));
  }

  // Writes a 32-bit integer
  BufferWriter& writeI32(int32_t data) {
    return writeBytes(reinterpret_cast<const uint8_t*>(&data), sizeof(data));
  }

  // Writes a 16-bit integer
  BufferWriter& writeI16(int16_t data) {
    return writeBytes(reinterpret_cast<const uint8_t*>(&data), sizeof(data));
  }

  // Writes an 8-bit integer
  BufferWriter& writeI8(int8_t data) {
    return writeBytes(reinterpret_cast<const uint8_t*>(&data), sizeof(data));
  }

  // Writes an unsigned 64-bit integer
  BufferWriter& writeU64(uint64_t data) {
    return writeBytes(reinterpret_cast<const uint8_t*>(&data), sizeof(data));
  }

  // Writes an unsigned 32-bit integer
  BufferWriter& writeU32(uint32_t data) {
    return writeBytes(reinterpret_cast<const uint8_t*>(&data), sizeof(data));
  }

  // Writes an unsigned 16-bit integer
  BufferWriter& writeU16(uint16_t data) {
    return writeBytes(reinterpret_cast<const uint8_t*>(&data), sizeof(data));
  }

  // Writes an unsigned 8-bit integer
  BufferWriter& writeU8(uint8_t data) {
    return writeBytes(reinterpret_cast<const uint8_t*>(&data), sizeof(data));
  }

  // Writes a boolean
  BufferWriter& writeBool(bool data) {
    return writeU8(data ? 1 : 0);
  }

  // Returns the current buffer
  const uint8_t* getBuffer() const {
    return buffer;
  }

  // Returns the current write position
  size_t getSize() const {
    return writePos;
  }

  // Clears the buffer
  void clear() {
    writePos = 0;
  }
};

class BufferReader {
private:
  const uint8_t* buffer;
  size_t bufferSize;
  size_t readPos;
public:
  BufferReader(const uint8_t* buffer, size_t bufferSize)
    : buffer(buffer), bufferSize(bufferSize), readPos(0) {}

  bool readBytes(uint8_t* output, size_t length) {
    if (readPos + length > bufferSize) {
      return false;
    }
    memcpy(output, buffer + readPos, length);
    readPos += length;
    return true;
  }

  bool readI64(int64_t& value) {
    uint8_t bytes[8];
    if (!readBytes(bytes, sizeof(bytes))) return false;
    value = *(reinterpret_cast<int64_t*>(bytes));
    return true;
  }

  bool readI32(int32_t& value) {
    uint8_t bytes[4];
    if (!readBytes(bytes, sizeof(bytes))) return false;
    value = *(reinterpret_cast<int32_t*>(bytes));
    return true;
  }

  bool readI16(int16_t& value) {
    uint8_t bytes[2];
    if (!readBytes(bytes, sizeof(bytes))) return false;
    value = *(reinterpret_cast<int16_t*>(bytes));
    return true;
  }

  bool readI8(int8_t& value) {
    uint8_t byte;
    if (!readBytes(&byte, 1)) return false;
    value = *(reinterpret_cast<int8_t*>(&byte));
    return true;
  }

  bool readString(String& output) {
    uint32_t length;
    if (!readU32(length)) return false;

    if (readPos + length > bufferSize) return false;

    output = String((const char*)(buffer + readPos)).substring(0, length);

    readPos += length;
    return true;
  }

  bool readU64(uint64_t& value) {
    uint8_t bytes[8];
    if (!readBytes(bytes, sizeof(bytes))) return false;
    value = *(reinterpret_cast<uint64_t*>(bytes));
    return true;
  }

  bool readU32(uint32_t& value) {
    uint8_t bytes[4];
    if (!readBytes(bytes, sizeof(bytes))) return false;
    value = *(reinterpret_cast<uint32_t*>(bytes));
    return true;
  }

  bool readU16(uint16_t& value) {
    uint8_t bytes[2];
    if (!readBytes(bytes, sizeof(bytes))) return false;
    value = *(reinterpret_cast<uint16_t*>(bytes));
    return true;
  }

  bool readU8(uint8_t& value) {
    if (!readBytes(&value, 1)) return false;
    return true;
  }

  bool readBool(bool& value) {
    uint8_t byte;
    if (!readU8(byte)) return false;
    value = (byte != 0);
    return true;
  }

  size_t getReadPos() const {
    return readPos;
  }
};

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

unsigned long previousMillis = 0;  // stores the last time the event was triggered
const long interval = 5000;         // interval in milliseconds (1 second)
int wifiTimeout = 0;
bool deviceIdRequestPending = false;

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
    while (1)
      ;
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

  switch (setupNetwork()) {
    case NetworkStatus::WiFiError:
      data.wifi = false;
      EEPROM.put(0, data);
    case NetworkStatus::ServerError:
      Serial << "can't connect to the server" << endl;
    case NetworkStatus::Ok:
      break;
    default:
      break;
  }
}

void send_packet(ClientPacket id, uint32_t len, const uint8_t* buffer) {
  BufferWriter header_writer(8);
  header_writer.writeU32(len);
  header_writer.writeU32(static_cast<uint32_t>(id));
  client.write(header_writer.getBuffer(), header_writer.getSize());
  client.write(buffer, len);
}

void process_server_packet(ServerPacket id, BufferReader reader) {
  switch (id) {
    case ServerPacket::ResponseId:
      reader.readBytes((uint8_t*)&data.device_id, 64);
      data.device = 1;
      EEPROM.put(0, data);
      break;
    case ServerPacket::UpdateCooler:
      break;
    case ServerPacket::WaterPulse:
      break;
  }
}

void receive_packet() {
  uint8_t header[8];
  client.read(header, 8);
  BufferReader reader(header, 8);
  uint32_t length;
  reader.readU32(length);
  uint32_t id;
  reader.readU32(id);
  uint8_t* buffer = (uint8_t*)malloc(length);
  client.read(buffer, length);
  Serial << "Receved server packet id: " << id << endl;
  process_server_packet(static_cast<ServerPacket>(id), BufferReader(buffer, length));
  free(buffer);
}

void sendJsonBLE(JsonDocument& document) {
  String data;
  serializeJson(document, data);
  berryResponseCharacteristic.setValue(data);
}


NetworkStatus setupNetwork() {
  if (!data.wifi) return NetworkStatus::NoWiFI;
  int timeout = 0;
  while (WiFi.status() != WL_CONNECTED && timeout < 3) {
    Serial << "Connecting to the wifi" << endl;
    WiFi.begin((const char*)&data.wifi_ssid, (const char*)&data.wifi_password);
    delay(1000);
    timeout++;
  }
  if (timeout >= 3) {
    return NetworkStatus::WiFiError;
  }

  int server_timeout = 0;
  while (!client.connected() && server_timeout < 3) {
    Serial << "Connecting to the server" << endl;
    client.connect("35.198.240.174", 4000);
    delay(1000);
    server_timeout++;
  }

  if (server_timeout >= 3) {
    return NetworkStatus::ServerError;
  }

  if (data.device) {
    BufferWriter writer;
    writer.writeBytes((const uint8_t*)&data.device_id, 64);
    send_packet(ClientPacket::ReportId, writer.getSize(), writer.getBuffer());
  }

  return NetworkStatus::Ok;
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
          } else if (request.equals("device")) {
            JsonDocument doc;
            if (data.device == false) {
              doc["need_id"] = true;
            } else {
              doc["need_id"] = false;
            }
            sendJsonBLE(doc);
          } else if (request.equals("wifi-set")) {
            String ssid = doc["ssid"];
            String password = doc["password"];
            data.wifi = true;
            strncpy((char*)&data.wifi_ssid, ssid.c_str(), sizeof(data.wifi_ssid));
            strncpy((char*)&data.wifi_password, password.c_str(), sizeof(data.wifi_password));
            EEPROM.put(0, data);
          } else if (request.equals("device-set")) {
            String id = doc["device_id"];
            data.device = true;
            strncpy((char*)&data.device_id, id.c_str(), sizeof(data.device_id));
            EEPROM.put(0, data);
          } else if (request.equals("device-get")) {
            JsonDocument doc;
            doc["device_id"] = data.device_id;
            sendJsonBLE(doc);
          } else if (request.equals("close")) {
            central.disconnect();
          }
        }
      }
    }
  }

  if (WiFi.status() == WL_CONNECTED) {
    if (!client.connected()) {
      if (client.connect("35.198.240.174", 4000)) {
        Serial << "Connection Successful" << endl;
        if (data.device) {
          BufferWriter writer;
          writer.writeBytes((const uint8_t*)&data.device_id, 64);
          send_packet(ClientPacket::ReportId, writer.getSize(), writer.getBuffer());
        }
      } else {
        Serial << "Retrying to connect to the server..." << endl;
        delay(1000);
      }
    } else {
        if (millis() - previousMillis >= interval) {
          previousMillis = millis();
          BufferWriter writer;
          writer.writeU16(15);
          writer.writeU16(12);
          writer.writeU16(20);
          writer.writeU64(128);
          send_packet(ClientPacket::ReportSensors, writer.getSize(), writer.getBuffer()); 
          BufferWriter writerr;
          writerr.writeU64(128);
          uint8_t a[128];
          writerr.writeBytes(a, 128);
          send_packet(ClientPacket::ImageFrame, writerr.getSize(), writerr.getBuffer()); 
        }
      if (client.available()) {
        receive_packet();
      }
    }
  } else if (wifiTimeout < 3 && data.wifi) {
    Serial << "Connecting to wifi. retry count: " << wifiTimeout << endl;
    WiFi.begin((const char*)&data.wifi_ssid, (const char*)&data.wifi_password);
    delay(1000);
    wifiTimeout++;
  } else if (wifiTimeout >= 3 && data.wifi) {
    data.wifi = false;
    wifiTimeout = 0;
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
