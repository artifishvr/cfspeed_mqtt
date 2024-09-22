use cfspeedtest::speedtest::speed_test;
use cfspeedtest::speedtest::PayloadSize;
use cfspeedtest::OutputFormat;
use cfspeedtest::SpeedTestCLIOptions;
use dotenv::dotenv;
use reqwest;
use rumqttc::{Client, MqttOptions, QoS};
use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    dotenv().ok();

    let mqtt_host = env::var("MQTT_HOST").expect("Missing MQTT_HOST env var");
    let mqtt_port = env::var("MQTT_PORT").expect("Missing MQTT_PORT env var");
    let mqtt_upload_topic =
        env::var("MQTT_UPLOAD_TOPIC").expect("Missing MQTT_UPLOAD_TOPIC env var");
    let mqtt_download_topic =
        env::var("MQTT_DOWNLOAD_TOPIC").expect("Missing MQTT_DOWNLOAD_TOPIC env var");

    loop {
        // Define speedtest options
        let options = SpeedTestCLIOptions {
            output_format: OutputFormat::None, // don't write to stdout
            ipv4: false,                       // don't force ipv4 usage
            ipv6: false,                       // don't force ipv6 usage
            verbose: false,
            nr_tests: 5,
            nr_latency_tests: 20,
            max_payload_size: PayloadSize::M10,
            disable_dynamic_max_payload_size: false,
        };

        // Perform speed test
        let measurements = speed_test(reqwest::blocking::Client::new(), options);

        // Calculate average upload and download speeds
        let mut upload_speeds = Vec::new();
        let mut download_speeds = Vec::new();
        for measurement in &measurements {
            if measurement.test_type == cfspeedtest::speedtest::TestType::Upload {
                upload_speeds.push(measurement.mbit);
            }

            if measurement.test_type == cfspeedtest::speedtest::TestType::Download {
                download_speeds.push(measurement.mbit);
            }
        }

        // Calculate 90th percentile
        upload_speeds.sort_by(|a, b| a.partial_cmp(b).unwrap());
        download_speeds.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let upload_90th_percentile = upload_speeds[(upload_speeds.len() as f64 * 0.9) as usize];
        let download_90th_percentile =
            download_speeds[(download_speeds.len() as f64 * 0.9) as usize];

        // Send results over MQTT
        let mut mqttoptions =
            MqttOptions::new("cfspeed", mqtt_host.clone(), mqtt_port.parse().unwrap());
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, mut connection) = Client::new(mqttoptions, 10);

        client
            .publish(
                mqtt_download_topic.clone(),
                QoS::AtLeastOnce,
                false,
                download_90th_percentile.round().to_string(),
            )
            .unwrap();

        client
            .publish(
                mqtt_upload_topic.clone(),
                QoS::AtLeastOnce,
                false,
                upload_90th_percentile.round().to_string(),
            )
            .unwrap();

        client.disconnect().unwrap();

        // Iterate to poll the eventloop for connection progress
        for (i, notification) in connection.iter().enumerate() {
            println!("Notification = {:?}", notification);

            if i > 5 {
                break;
            }
        }

        println!("Test done, sleeping...");
        thread::sleep(Duration::from_secs(30));
    }
}
