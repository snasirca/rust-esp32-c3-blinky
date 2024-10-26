use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::bail;
use anyhow::Result;

use embedded_svc::mqtt::client::utils::ConnState;
use log::*;

use embedded_svc::mqtt::client::{Connection, MessageImpl, Publish, QoS};
use esp_idf_svc::mqtt::client::*;

use embedded_hal::digital::blocking::OutputPin;
use esp_idf_hal::peripherals::Peripherals;

use embedded_svc::wifi::*;

use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::wifi::*;

use esp_idf_sys::EspError;

const WIFI_SSID: &str = env!("RUST_ESP32_WIFI_SSID");
const WIFI_PASS: &str = env!("RUST_ESP32_WIFI_PASS");
const MQTT_HOST: &str = env!("RUST_ESP32_MQTT_HOST");
const MQTT_PORT: &str = env!("RUST_ESP32_MQTT_PORT");

fn main() {
    sensible_env_logger::init!();

    let peripherals = Peripherals::take().unwrap();
    let mut led = peripherals.pins.gpio4.into_output().unwrap();

    let netif_stack = Arc::new(EspNetifStack::new().unwrap());
    let sys_loop_stack = Arc::new(EspSysLoopStack::new().unwrap());
    let default_nvs = Arc::new(EspDefaultNvs::new().unwrap());

    let _wifi = start_wifi_client(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    )
    .unwrap();
    let _mqtt_client = start_mqtt_client().unwrap();

    led.set_high().unwrap();
    thread::sleep(Duration::from_millis(1000));

    led.set_low().unwrap();
    thread::sleep(Duration::from_millis(1000));

    println!("HELLO WORLD!");

    unsafe {
        info!("About to get to sleep now. Will wake up automatically in 5 seconds");
        esp_idf_sys::esp_deep_sleep(Duration::from_secs(5).as_micros() as u64);
    }
}

fn start_wifi_client(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> Result<Box<EspWifi>> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    info!("Wifi created");

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: WIFI_SSID.into(),
        password: WIFI_PASS.into(),
        ..Default::default()
    }))?;

    info!("Wifi configuration set, about to get status");

    wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
        .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;

    let status = wifi.get_status();

    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(
            _ip_settings,
        ))),
        ApStatus::Stopped,
    ) = status
    {
        info!("Wifi connected");
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}

fn start_mqtt_client() -> Result<EspMqttClient<ConnState<MessageImpl, EspError>>> {
    info!("About to start MQTT client");

    let conf = MqttClientConfiguration {
        client_id: Some("rust-esp32-c3-blinky"),
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),

        ..Default::default()
    };

    let mqtt_url = format!("mqtt://{}:{}", MQTT_HOST, MQTT_PORT);
    let (mut client, mut connection) = EspMqttClient::new_with_conn(mqtt_url, &conf)?;

    info!("MQTT client started");

    // See this comment: https://github.com/ivmarkov/rust-esp32-std-demo/blob/main/src/main.rs#L636
    thread::spawn(move || {
        info!("MQTT Listening for messages");

        while let Some(msg) = connection.next() {
            match msg {
                Err(e) => info!("MQTT Message ERROR: {}", e),
                Ok(msg) => info!("MQTT Message: {:?}", msg),
            }
        }

        info!("MQTT connection loop exit");
    });

    client.publish(
        "rust-esp32-c3-blinky",
        QoS::AtMostOnce,
        false,
        "Hello from rust-esp32-c3-blinky!".as_bytes(),
    )?;

    info!("Published a hello message to topic \"rust-esp32-c3-blinky\"");

    Ok(client)
}
