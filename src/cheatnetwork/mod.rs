use crate::document::action;
use serde_json::Value;
use thirtyfour::prelude::*;

pub mod answer;
pub async fn login(
    driver: &WebDriver,
    old_handle: &WindowHandle,
    data: &Value,
) -> WebDriverResult<()> {
    let new_handle = driver.new_tab().await?;
    driver.switch_to_window(new_handle).await?;
    driver.goto("https://cheatnetwork.eu/login").await?;

    action::send_keys(
        driver,
        &By::Tag("input"),
        data["backup-key"].as_str().unwrap(),
    )
    .await?;
    action::click_from(driver, &By::Tag("button"), 2, 10).await?;

    driver.close_window().await?;
    driver.switch_to_window(old_handle.clone()).await?;

    Ok(())
}
