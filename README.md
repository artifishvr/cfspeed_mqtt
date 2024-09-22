# Cloudflare Speed Tests sent over MQTT

This is a simple Rust program that performs a Cloudflare speed test on a user defined interval and sends the results over MQTT.

Useful to integrate with home assistant for speed test stats on your dashboard.

![screenshot of home assitant dashboard with upload and download speeds showing on a graph](homeassistant.png)

Inspired by [simonjenny/fastcom-mqtt](https://github.com/simonjenny/fastcom-mqtt), remade in Rust and using Cloudflare's Speedtest instead of Fast.com.

## Usage

i can't get docker to build right now lol
