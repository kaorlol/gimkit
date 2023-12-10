use crate::document::action;
use serde_json::Value;
use thirtyfour::prelude::*;

pub mod answer;
pub async fn login(
    driver: &WebDriver,
    old_handle: &WindowHandle,
    data: &Value,
) -> WebDriverResult<()> {
    let tab_handle = driver.new_tab().await?;
    driver.switch_to_window(tab_handle).await?;
    driver.goto("https://cheatnetwork.eu/login").await?;

    let input = By::Tag("input");
    let button = By::Tag("button");

    let backup_key = data["backup-key"].as_str().unwrap();
    action::send_keys(driver, &input, backup_key).await?;
    action::click_from(driver, &button, 2).await?;

    driver.close_window().await?;
    driver.switch_to_window(old_handle.clone()).await?;

    Ok(())
}
