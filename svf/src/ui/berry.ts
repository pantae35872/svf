import Swal from "sweetalert2";

async function waitForNotification(characteristic: BluetoothRemoteGATTCharacteristic): Promise<DataView> {
  return new Promise((resolve, reject) => {
    const onNotification = (event: Event) => {
      const target = event.target as BluetoothRemoteGATTCharacteristic;
      target?.removeEventListener("characteristicvaluechanged", onNotification); // Unsubscribe to avoid multiple triggers
      if (target.value) {
        resolve(target.value);
      } else {
        reject(new Error("No value received in notification"));
      }
    };

    // Attach the event listener for notifications
    characteristic.addEventListener("characteristicvaluechanged", onNotification);

    // Ensure notifications are started
    characteristic.startNotifications().catch(reject);
  });
}


async function get_ssid_password(): Promise<{ ssid: string | null, password: string | null }> {
  const ssid = await Swal.fire({
    title: "Enter wifi name for your BerryBotics.",
    input: "text",
    confirmButtonText: "Continue",
    color: "var(--fg-color)",
    background: "var(--bg-color-4)",
    confirmButtonColor: "var(--bg-color-3)",
    customClass: {
      input: 'alert-input-style',
      confirmButton: 'confirm-button-style',
      cancelButton: 'cancel-button-style',
    },
    inputAttributes: {
      autocapitalize: "off",
    },
    allowOutsideClick: false,
  });
  const password = await Swal.fire({
    title: "Enter wifi password for your BerryBotics.",
    input: "password",
    confirmButtonText: "Continue",
    color: "var(--fg-color)",
    background: "var(--bg-color-4)",
    confirmButtonColor: "var(--bg-color-3)",
    customClass: {
      input: 'alert-input-style',
      confirmButton: 'confirm-button-style',
      cancelButton: 'cancel-button-style',
    },
    inputAttributes: {
      autocapitalize: "off",
    },
    allowOutsideClick: false,
  });

  if (typeof (password) == 'string' && typeof (ssid) == 'string') {
    return {
      ssid: ssid,
      password: password,
    };
  }

  return { ssid: null, password: null };
}

export async function request_berry(): Promise<string | null> {
  const SERVICE_UUID = 'f901b2a6-02a1-40ab-8b44-6471bd5886af'; // Battery Service UUID
  const REQUEST_CHARACTERISTIC_UUID = '9fbdeb54-dab7-42a0-bca1-6f6a80240c45';
  const RESPONSE_CHARACTERISTIC_UUID = '3457d261-fdf2-4e43-98f6-6f9064ff8abd';
  let data: string | null = null;
  try {
    const device = await navigator.bluetooth.requestDevice({
      filters: [{ services: [SERVICE_UUID] }],
    });

    device.addEventListener('gattserverdisconnected', (event) => {
      console.log("Device disconnected:", event.target);
    });
    const server = await device.gatt?.connect();
    const service = await server?.getPrimaryService(SERVICE_UUID);
    const request_characteristic = await service?.getCharacteristic(REQUEST_CHARACTERISTIC_UUID);
    const response_characteristic = await service?.getCharacteristic(RESPONSE_CHARACTERISTIC_UUID);
    await response_characteristic?.startNotifications();
    if (response_characteristic) {
      while (true) {
        const value = await waitForNotification(response_characteristic);
        const decoder = new TextDecoder();
        const message = decoder.decode(value);
        console.log(message);
        let msg: { wifi_request: string } | { device_id: string } = JSON.parse(message);
        if ("wifi_request" in msg) {
          let credential = await get_ssid_password();
          if (credential.ssid && credential.password) {
            const message = JSON.stringify({
              ssid: credential.ssid,
              password: credential.password,
            });
            const encoder = new TextEncoder();
            const data = encoder.encode(message);
            await request_characteristic?.writeValue(data);
          }
        } else if ("device_id" in msg) {
          const message = JSON.stringify({
            close: "",
          });
          const encoder = new TextEncoder();
          const data = encoder.encode(message);
          await request_characteristic?.writeValue(data);
          return msg.device_id;
        }
      }
    }
  } catch (error) {
    console.log(error);
    Swal.fire({
      title: "Error",
      icon: 'error',
      color: "var(--fg-color)",
      text: "Failed to pair with the BerryBotics",
      background: "var(--bg-color-4)",
      customClass: {
        confirmButton: 'confirm-button-style',
      }
    });
  }
  return null;
}
