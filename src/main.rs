use std::time::Duration;

use gimkit::{
	gim_tools::{
		assignment::{auto_answer, start_assignment},
		login,
	},
	utils::is_gimkit_link,
};

use chromedriver_manager::{loglevel::LogLevel, manager::Handler};
use thirtyfour::prelude::*;
use tokio::time::sleep;

// TODO: Support for progress bar
// TODO: Rewrite code to be better and more optimized
// TODO: Add cookie handling (too login with cookies)

#[tokio::main]
async fn main() -> WebDriverResult<()> {
	let args = std::env::args().collect::<Vec<String>>();
	match args.len() {
		2 => {
			if !is_gimkit_link(&args[1]) {
				println!("Invalid gimkit link");
				return Ok(());
			}
		}
		_ => {
			println!("Usage: {} <assignment link>", args[0]);
			return Ok(());
		}
	}

	let mut caps = DesiredCapabilities::chrome();
	caps.add_chrome_arg("--mute-audio")?;
	// caps.set_headless()?;

	Handler::new()
		.launch_chromedriver(&mut caps, "9515", LogLevel::Off)
		.await
		.expect("Failed to start chromedriver");

	let driver = WebDriver::new("http://localhost:9515", caps).await?;
	driver.goto(&args[1]).await?;

	login(&driver).await?;
	start_assignment(&driver).await?;

	driver
		.execute(include_str!("gim_tools/answer_scraper.js"), vec![])
		.await?;

	sleep(Duration::from_secs(1)).await;

	auto_answer(&driver).await?;

	driver.quit().await?;

	Ok(())
}
