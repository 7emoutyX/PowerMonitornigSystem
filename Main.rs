// CREATED : 22/08/2023 - CNESTEN
// DAOUDI MOHAMMED 
// Contact me if u need more additional details 
// Email : Muhamedaoudi@gmail.com


use adafruit_tsl2561_unified::TSL2561;
use esp8266::wifi::WiFi;
use esp8266::http_client::HTTPClient;
use esp8266::json::JSON;
use esp8266::ina219::INA219;
use esp8266::fs::SPIFFS;
use esp8266::async_web_server::AsyncWebServer;
use esp8266::async_event_source::AsyncEventSource;

let server_name = "https://bobafettesp.000webhostapp.com/post.php";
let ina219 = INA219::new();
let ina220 = INA219::new(0x41);
let ssid = "7emouty";
let password = "bobafettX";
let server = AsyncWebServer::new(80);
let events = AsyncEventSource::new("/events");
let sensor_name = "BME280";
let sensor_location = "ENSA";
let mut readings = JSON::new();
let mut postdata = String::new();
let mut luminosite: f32;
let mut power1 = 0.0;
let mut current_mA = 0.0;
let mut power2 = 0.0;
let mut current_mA2 = 0.0;
let mut busvoltage = 0.0;
let mut busvoltage2 = 0.0;
let mut last_time = 0;
let timer_delay = 3000;
let tsl2561 = TSL2561::new(TSL2561_ADDR_FLOAT, 12345);
let mut event = tsl2561.get_event();
let api_key_value = "tPmAT5Ab3j7F9";

fn voltage_a() -> String {
    let p = ina219.get_bus_voltage_v();
    p.to_string()
}

fn lux() -> String {
    let event = tsl2561.get_event();
    if event.light != 0 {
        luminosite = event.light;
        luminosite.to_string()
    } else {
        "error".to_string()
    }
}

fn powerr() -> String {
    power1 = ina219.get_current_mA() * ina219.get_bus_voltage_v();
    power1.to_string()
}

fn sensor_data() -> String {
    readings["power1"] = ina219.get_power_mW().to_string();
    readings["power2"] = ina220.get_power_mW().to_string();
    // Add more readings here
    readings.to_string()
}
use std::collections::HashMap;

fn sensor_data() -> String {
    let mut readings: HashMap<String, String> = HashMap::new();
    readings.insert("power1".to_string(), ina219.get_power_mW().to_string());
    readings.insert("power2".to_string(), ina220.get_power_mW().to_string());
    readings.insert("luminosityData".to_string(), lux().to_string());
    let json_string = serde_json::to_string(&readings).unwrap();
    json_string
}

fn init_wifi() {
    wifi::init_sta();
    wifi::begin(ssid, password);
    print!("Connecting to WiFi ..");
    while wifi::status() != wifi::Status::Connected {
        print!(".");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("{}", wifi::local_ip());
}

fn setup() {
    serial::begin(115200);
    if !spiffs::begin() {
        println!("An Error has occurred while mounting SPIFFS");
        return;
    }
    if !ina219.begin() {
        println!("Failed to find INA219 chip");
        loop {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    if !ina220.begin() {
        println!("Failed to find INA220 chip");
        loop {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    init_wifi();
    tsl2561.enable_auto_range(true);
    tsl2561.set_integration_time(tsl2561::IntegrationTime::IntegrationTime13ms);
    server.on("/", http::Method::Get, |request| {
        request.send(spiffs, "/index.html");
    });
    server.serve_static("/", spiffs, "/");
    server.on("/readings", http::Method::Get, |request| {
        last_time = last_time + timer_delay;
        request.send(200, "text/plain", "OK!");
    });
    events.on_connect(|client| {
        if let Some(last_id) = client.last_id() {
            println!("Client reconnected! Last message ID that it got is: {}", last_id);
        }
    });
}
fn main() {
    if WiFi::status() == WL_CONNECTED {
        let mut client = WiFiClientSecure::new();
        client.set_insecure(); // don't use SSL certificate
        let mut https = HTTPClient::new();
        https.begin(&mut client, serverName);
        https.add_header("Content-Type", "application/x-www-form-urlencoded");
        let httpRequestData = format!("api_key={}&sensor={}&location={}&PowerA={}&PowerB={}&luminosité={}",
                                      apiKeyValue, sensorName, sensorLocation, ina219.getPower_mW(),
                                      ina220.getPower_mW(), lux());
        println!("httpRequestData: {}", httpRequestData);
        println!("httpRequestData: {}", httpRequestData);
        let httpResponseCode = https.POST(&httpRequestData);
        if httpResponseCode > 0 {
            println!("HTTP Response code: {}", httpResponseCode);
            println!("{}", ina220.getPower_mW());
        } else {
            println!("Error code: {}", httpResponseCode);
        }
        // Free resources
        https.end();
    } else {
        println!("WiFi Disconnected");
    }
    if (millis() - lastTime) > timerDelay {
        // Send Events to the client with the Sensor Readings Every 10 seconds
        events.send("ping", None, millis());
        events.send(SensorData().to_string().as_str(), "new_readings", millis());
        lastTime = millis();
    }
    delay(300);
}
