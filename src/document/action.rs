use crate::document::info::{query, query_all};
use thirtyfour::prelude::*;

pub async fn click(driver: &WebDriver, by: &By) -> WebDriverResult<()> {
    query(driver, by).await?.click().await?;

    Ok(())
}

pub async fn click_from(driver: &WebDriver, by: &By, index: usize) -> WebDriverResult<()> {
    let elements = query_all(driver, by).await?;
    elements
        .get(index)
        .unwrap_or_else(|| {
            panic!(
                "No element found with index {:?}, found elements: {:?}",
                index, elements
            )
        })
        .click()
        .await?;

    Ok(())
}

pub async fn send_keys(driver: &WebDriver, by: &By, keys: &str) -> WebDriverResult<()> {
    query(driver, by).await?.send_keys(keys).await?;

    Ok(())
}

pub async fn send_keys_from(
    driver: &WebDriver,
    by: &By,
    index: usize,
    keys: &str,
) -> WebDriverResult<()> {
    query_all(driver, by)
        .await?
        .get(index)
        .unwrap()
        .send_keys(keys)
        .await?;

    Ok(())
}
