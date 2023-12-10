use async_recursion::async_recursion;
use thirtyfour::prelude::*;
use crate::document::info::{query, query_all};

#[async_recursion]
pub async fn get_text(driver: &WebDriver, by: &By, text: &str) -> WebDriverResult<String> {
    let element = query(driver, by).await?;
    let element_text = element.text().await?;

    if element_text == text {
        return get_text(driver, by, text).await
    }

    Ok(element_text)
}

#[async_recursion]
pub async fn get_value(driver: &WebDriver, by: &By, value: &str) -> WebDriverResult<String> {
    let element = query(driver, by).await?;
    let element_value = element.value().await?.unwrap();

    if element_value == value {
        return get_value(driver, by, value).await
    }

    Ok(element_value)
}

#[async_recursion]
pub async fn get_text_from(driver: &WebDriver, by: &By, index: usize, text: &str, timeout: u64) -> WebDriverResult<String> {
    let elements = query_all(driver, by, timeout).await?;
    let element_text = elements.get(index).unwrap().text().await?;

    if element_text == text {
        return get_text_from(driver, by, index, text, timeout).await
    }

    Ok(element_text)
}

#[async_recursion]
pub async fn get_value_from(driver: &WebDriver, by: &By, index: usize, value: &str, timeout: u64) -> WebDriverResult<String> {
    let elements = query_all(driver, by, timeout).await?;
    let element_value = elements.get(index).unwrap().value().await?.unwrap();

    if element_value == value {
        return get_value_from(driver, by, index, value, timeout).await
    }

    Ok(element_value)
}