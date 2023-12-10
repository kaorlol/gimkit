use gimkit::{
    cheatnetwork::answer::get_answers,
    gim_tools::{
        assignment::{auto_answer, start_assignment},
        login,
    },
};

use chromedriver_manager::{loglevel::LogLevel, manager::Handler};
use thirtyfour::prelude::*;

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
    caps.add_chrome_arg("--mute-audio")?;
    // caps.set_headless()?;

    Handler::new()
        .launch_chromedriver(&mut caps, "9515", LogLevel::Off)
        .await
        .expect("Failed to start chromedriver");

    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto(&args[1]).await?;

    let data = login(&driver).await?;
    start_assignment(&driver).await?;

    let old_handle = driver.window().await?;
    let answers = get_answers(&driver, &old_handle, data).await?;

    auto_answer(&driver, &answers).await?;

    driver.quit().await?;

    Ok(())
}
