# Weather-CLI ☀️

A simple and fast command-line tool to fetch the current weather for any city, powered by Rust.

![rust-badge](https://img.shields.io/badge/language-Rust-orange.svg)

## Description

`weather-cli` provides a quick way to check the weather directly from your terminal. It uses the [OpenWeatherMap API](https://openweathermap.org/api) to fetch real-time weather data and displays it in a clean, readable format.

## Features

-   **Real-time Weather**: Get up-to-date weather information.
-   **Global Coverage**: Fetch weather for any city in the world.
-   **Detailed Info**: Displays temperature, "feels like" temperature, humidity, wind speed, and weather conditions.
-   **Asynchronous**: Built with Tokio for non-blocking I/O, ensuring it runs quickly.

## Setup and Installation

1.  **Get an API Key**:
    -   Sign up for a free account on [OpenWeatherMap](https://openweathermap.org/appid).
    -   Generate an API key from your account dashboard.

2.  **Clone the repository**:
    ```sh
    git clone https://github.com/your-username/weather-cli.git
    cd weather-cli
    ```

3.  **Create `.env` file**:
    -   Create a file named `.env` in the root of the project.
    -   Add your API key to it like this:
        ```
        OPENWEATHERMAP_API_KEY="your_api_key_here"
        ```

4.  **Build the project**:
    ```sh
    cargo build --release
    ```
    The executable will be at `target/release/weather-cli`.

## Usage

Run the application with the city name as an argument.

```sh
cargo run -- London
# or with country code
cargo run -- "New York,US"
