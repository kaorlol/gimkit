use crate::document::action;
use colored::Colorize;
use serde_json::Value;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Instant};

pub mod assignment;
pub async fn login(driver: &WebDriver) -> WebDriverResult<Value> {
    let start = Instant::now();

    println!("{}", "Getting user credentials...".blue());
    let credentials = include_str!("..\\..\\credentials.json");
    let data: Value = serde_json::from_str(&credentials)?;

    let input = By::Tag("input");
    let button = By::Tag("button");

    println!("{}", "Logging in...".blue());
    action::click_from(driver, &By::Tag("a"), 1).await?;

    let email = data["email"].as_str().unwrap();
    action::send_keys(driver, &input, email).await?;
    println!("{}", "Email entered".blue());

    action::click_from(driver, &button, 1).await?;

    sleep(Duration::from_secs_f64(0.5)).await;

    let password = data["password"].as_str().unwrap();
    action::send_keys(driver, &input, password).await?;
    println!("{}", "Password entered".blue());

    action::click(driver, &button).await?;

    let time = format!("(took {:.2}s)", start.elapsed().as_secs_f32());
    println!("{}", format!("{} {}", "Logged in".blue(), time.dimmed()));

    Ok(data)
}
