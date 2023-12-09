use std::time::{Duration, Instant};
use serde_json::Value;
use thirtyfour::prelude::*;
use tokio::time::sleep;
use colored::Colorize;
use crate::document::*;

pub async fn login(driver: WebDriver) -> WebDriverResult<Value> {
	let start = Instant::now();
	println!("{}", "Getting user credentials...".blue());
	let credentials = include_str!("..\\..\\credentials.json");
	let data: Value = serde_json::from_str(&credentials)?;

	println!("{}", "Logging in...".blue());
	actions::click_from(driver.clone(), By::Tag("a"), 1, 10).await?;

	actions::send_keys(driver.clone(), By::Tag("input"), data["email"].as_str().unwrap()).await?;
	println!("{}", "Email entered".blue());

	actions::click_from(driver.clone(), By::Tag("button"),1,  10).await?;
	println!("{}", "Continuing...".blue());

	sleep(Duration::from_secs_f64(0.5)).await;

	actions::send_keys(driver.clone(), By::Tag("input"), data["password"].as_str().unwrap()).await?;
	println!("{}", "Password entered".blue());

	actions::click(driver.clone(), By::Tag("button")).await?;
	println!("{}", format!("{} {}", "Logged in".blue(), format!("(took {}s)", start.elapsed().as_secs()).dimmed()));

	Ok(data)
}

pub async fn start_assignment(driver: WebDriver) -> WebDriverResult<()> {
	println!("{}", "\nStarting assignment...".blue());
	info::query(driver.clone(), By::Tag("svg")).await?;
	actions::click(driver.clone(), By::Css(".btn-pushable")).await?;
	println!("{}", "Assignment started\nLoading assignment...".blue());
	actions::click(driver.clone(), By::Css("div[class='sc-hdWpuu cCeQmZ']")).await?;

	sleep(Duration::from_secs(1)).await;

	Ok(())
}