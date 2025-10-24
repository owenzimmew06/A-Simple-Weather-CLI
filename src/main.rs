use clap::Parser;
use dotenv::dotenv;
use reqwest::Error;
use serde::Deserialize;
use std::env;

// Structs to deserialize the JSON response from OpenWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    feels_like: f64,
    humidity: u32,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// Define the command-line arguments using clap
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "A simple CLI to get weather information for a city")]
struct Cli {
    #[arg(help = "The city to get the weather for, e.g., 'London' or 'New York,US'")]
    city: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from .env file
    dotenv().ok();
    
    let cli = Cli::parse();
    let api_key = env::var("OPENWEATHERMAP_API_KEY")
        .expect("OPENWEATHERMAP_API_KEY must be set in your .env file");

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        cli.city, api_key
    );

    println!("Fetching weather for {}...", cli.city);

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let weather_data: WeatherResponse = response.json().await?;
        
        println!("\n--- Weather Report for {} ---", weather_data.name);
        if let Some(weather) = weather_data.weather.first() {
            println!("Condition: {}", weather.description);
        }
        println!("Temperature: {:.1}°C", weather_data.main.temp);
        println!("Feels like:  {:.1}°C", weather_data.main.feels_like);
        println!("Humidity:    {}%", weather_data.main.humidity);
        println!("Wind Speed:  {:.1} m/s", weather_data.wind.speed);
        println!("---------------------------------");

    } else {
        eprintln!("Error: Failed to fetch weather data. Status: {}", response.status());
        eprintln!("Please check if the city name is correct and your API key is valid.");
    }

    Ok(())
}
