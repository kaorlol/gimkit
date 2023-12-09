use thirtyfour::prelude::*;
use chromedriver_manager::manager::Handler;
use gimkit::{auto_answer::*, login::*, cheatnetwork::get_answers};

// TODO: Support for progress bar
// TODO: Rewrite code to be better and more optimized
// TODO: Add cookie handling (too login with cookies)

#[tokio::main]
async fn main() -> WebDriverResult<()> {
	let args = std::env::args().collect::<Vec<String>>();
	if args.len() < 2 {
		println!("Usage: {} <assignment link>", args[0]);
		return Ok(());
	}

	let mut caps = DesiredCapabilities::chrome();
	Handler::new().launch_chromedriver(&mut caps, false, "9515").await.expect("Failed to launch chromedriver");
	caps.add_chrome_arg("--mute-audio")?;
	// caps.set_headless()?;

	let driver = WebDriver::new("http://localhost:9515", caps).await?;
	driver.goto(&args[1]).await?;

	let data = login(driver.clone()).await?;
	start_assignment(driver.clone()).await?;

	let old_handle = driver.window().await?;
	let answers = get_answers(driver.clone(), old_handle, data).await?;

	auto_answer(driver.clone(), answers).await?;

	driver.quit().await?;

	Ok(())
}
