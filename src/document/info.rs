use thirtyfour::prelude::*;

pub async fn get_text(driver: &WebDriver, by: &By) -> WebDriverResult<String> {
    let element = query(driver, by).await?;
    let text = element.text().await?;
    Ok(text)
}

pub async fn get_value(driver: &WebDriver, by: &By) -> WebDriverResult<String> {
    let element = &query(driver, by).await?;
    let value = element.value().await?.unwrap_or_default();
    Ok(value)
}

pub async fn get_text_from(driver: &WebDriver, by: &By, index: usize) -> WebDriverResult<String> {
    let elements = query_all(driver, by).await?;
    let element = elements.get(index).unwrap();
    let text = element.text().await?;
    Ok(text)
}

pub async fn get_value_from(driver: &WebDriver, by: &By, index: usize) -> WebDriverResult<String> {
    let elements = query_all(driver, by).await?;
    let element = elements.get(index).unwrap();
    let value = element.value().await?.unwrap_or_default();
    Ok(value)
}

pub async fn find(driver: &WebDriver, by: &By) -> WebDriverResult<WebElement> {
    if let Err(_) = driver.find(by.clone()).await {
        return Err(WebDriverError::NoSuchElement(format!(
            "No element found with {:?}",
            by
        )));
    }

    let element = driver.find(by.clone()).await?;
    Ok(element)
}

pub async fn exists(driver: &WebDriver, by: &By) -> WebDriverResult<bool> {
    let elements = driver.find_all(by.clone()).await?;
    let length = elements.len();
    Ok(length > 0)
}

pub async fn query(driver: &WebDriver, by: &By) -> WebDriverResult<WebElement> {
    if let Err(_) = driver.query(by.clone()).and_displayed().first().await {
        return Err(WebDriverError::NoSuchElement(format!(
            "No element found with {:?}",
            by
        )));
    }

    let element_query = driver.query(by.clone());
    let element = element_query.and_displayed().first().await?;
    Ok(element)
}

pub async fn query_all(driver: &WebDriver, by: &By) -> WebDriverResult<Vec<WebElement>> {
    if let Err(_) = driver.query(by.clone()).and_displayed().all().await {
        return Err(WebDriverError::NoSuchElement(format!(
            "No element found with {:?}",
            by
        )));
    }

    let element_query = driver.query(by.clone());
    let elements = element_query.and_displayed().all().await?;
    Ok(elements)
}
