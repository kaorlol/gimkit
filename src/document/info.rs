use std::time::Duration;
use thirtyfour::prelude::*;

pub async fn get_text(driver: &WebDriver, by: &By) -> WebDriverResult<String> {
    Ok(query(driver, by).await?.text().await?)
}

pub async fn get_value(driver: &WebDriver, by: &By) -> WebDriverResult<String> {
    Ok(query(driver, by).await?.value().await?.unwrap())
}

pub async fn get_text_from(driver: &WebDriver, by: &By, index: usize, timeout: u64) -> WebDriverResult<String> {
    Ok(query_all(driver, by, timeout).await?.get(index).unwrap().text().await?)
}

pub async fn get_value_from(driver: &WebDriver, by: &By, index: usize, timeout: u64) -> WebDriverResult<String> {
    Ok(query_all(driver, by, timeout).await?.get(index).unwrap().value().await?.unwrap())
}

pub async fn find(driver: &WebDriver, by: &By) -> WebDriverResult<WebElement> {
    if let Err(_) = driver.find(by.clone()).await {
        driver.to_owned().quit().await?;

        return Err(WebDriverError::NoSuchElement(
            format!("No element found with {:?}", by)
        ));
    }

    Ok(driver.find(by.clone()).await?)
}

pub async fn exists(driver: &WebDriver, by: &By) -> WebDriverResult<bool> {
    Ok(driver.find_all(by.clone()).await?.len() > 0)
}

pub async fn query(driver: &WebDriver, by: &By) -> WebDriverResult<WebElement> {
    if let Err(_) = driver.query(by.clone()).and_displayed().first().await {
        driver.to_owned().quit().await?;

        return Err(WebDriverError::NoSuchElement(
            format!("No element found with {:?}", by)
        ));
    }

    Ok(driver.query(by.clone()).and_displayed().first().await?)
}

pub async fn query_all(driver: &WebDriver, by: &By, timeout: u64) -> WebDriverResult<Vec<WebElement>> {
    if let Err(_) = driver.query(by.clone()).and_displayed().wait(Duration::from_secs(timeout), Duration::from_secs(1)).all().await {
        driver.to_owned().quit().await?;

        return Err(WebDriverError::NoSuchElement(
            format!("No element found with {:?}", by)
        ));
    }

    Ok(driver.query(by.clone()).and_displayed().all().await?)
}