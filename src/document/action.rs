use thirtyfour::prelude::*;
use crate::document::info::{query, query_all};

pub async fn click(driver: &WebDriver, by: &By) -> WebDriverResult<()> {
    query(driver, by).await?.click().await?;

    Ok(())
}

pub async fn click_from(driver: &WebDriver, by: &By, index: usize, timeout: u64) -> WebDriverResult<()> {
    let elements = query_all(driver, by, timeout).await?;
    elements.get(index).unwrap_or_else(|| panic!("No element found with index {:?}, found elements: {:?}", index, elements)).click().await?;

    Ok(())
}

pub async fn send_keys(driver: &WebDriver, by: &By, keys: &str) -> WebDriverResult<()> {
    query(driver, by).await?.send_keys(keys).await?;

    Ok(())
}

pub async fn send_keys_from(driver: &WebDriver, by: &By, index: usize, keys: &str, timeout: u64) -> WebDriverResult<()> {
    query_all(driver, by, timeout).await?.get(index).unwrap().send_keys(keys).await?;

    Ok(())
}