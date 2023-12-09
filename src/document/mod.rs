use std::time::Duration;
use async_recursion::async_recursion;
use thirtyfour::prelude::*;

pub mod info {
    use super::*;

    pub mod recursion {
        use super::*;

        #[async_recursion]
        pub async fn get_text(driver: WebDriver, by: By, text: &str) -> WebDriverResult<String> {
            let element = query(driver.clone(), by.clone()).await?;
            let element_text = element.text().await?;
        
            if element_text == text {
                return get_text(driver.clone(), by, text).await
            }

            Ok(element_text)
        }

        #[async_recursion]
        pub async fn get_value(driver: WebDriver, by: By, value: &str) -> WebDriverResult<String> {
            let element = query(driver.clone(), by.clone()).await?;
            let element_value = element.value().await?.unwrap();
        
            if element_value == value {
                return get_value(driver.clone(), by, value).await
            }

            Ok(element_value)
        }
    }
    
    pub async fn get_text(driver: WebDriver, by: By) -> WebDriverResult<String> {
        Ok(query(driver.clone(), by).await?.text().await?)
    }

    pub async fn get_value(driver: WebDriver, by: By) -> WebDriverResult<String> {
        Ok(query(driver.clone(), by).await?.value().await?.unwrap())
    }
    
    pub async fn get_text_from(driver: WebDriver, by: By, index: usize, timeout: u64) -> WebDriverResult<String> {
        Ok(query_all(driver.clone(), by, timeout).await?.get(index).unwrap().text().await?)
    }

    pub async fn get_value_from(driver: WebDriver, by: By, index: usize, timeout: u64) -> WebDriverResult<String> {
        Ok(query_all(driver.clone(), by, timeout).await?.get(index).unwrap().value().await?.unwrap())
    }

    pub async fn find(driver: WebDriver, by: By) -> WebDriverResult<WebElement> {
        if let Err(_) = driver.find(by.clone()).await {
            return Err(WebDriverError::NoSuchElement(
                format!("No element found with {:?} selector", by)
            ));
        }
    
        Ok(driver.find(by).await?)
    }

    pub async fn exists(driver: WebDriver, by: By) -> WebDriverResult<bool> {
        Ok(driver.find_all(by).await?.len() > 0)
    }
    
    pub async fn query(driver: WebDriver, by: By) -> WebDriverResult<WebElement> {
        if let Err(_) = driver.query(by.clone()).and_displayed().first().await {
            return Err(WebDriverError::NoSuchElement(
                format!("No element found with {:?} selector", by)
            ));
        }
    
        Ok(driver.query(by).and_displayed().first().await?)
    }
    
    pub async fn query_all(driver: WebDriver, by: By, timeout: u64) -> WebDriverResult<Vec<WebElement>> {
        if let Err(_) = driver.query(by.clone()).and_displayed().wait(Duration::from_secs(timeout), Duration::from_secs(1)).all().await {
            return Err(WebDriverError::NoSuchElement(
                format!("No element found with {:?} selector", by)
            ));
        }
    
        Ok(driver.query(by).and_displayed().all().await?)
    }  
}

pub mod actions {
    use super::*;
    
    pub async fn click(driver: WebDriver, by: By) -> WebDriverResult<()> {
        info::query(driver.clone(), by).await?.click().await?;
    
        Ok(())
    }
    
    pub async fn click_from(driver: WebDriver, by: By, index: usize, timeout: u64) -> WebDriverResult<()> {
        let elements = info::query_all(driver.clone(), by.clone(), timeout).await?;
        elements.get(index).unwrap_or_else(|| panic!("No element found with index {:?}, found elements: {:?}", index, elements)).click().await?;
    
        Ok(())
    }
    
    pub async fn send_keys(driver: WebDriver, by: By, keys: &str) -> WebDriverResult<()> {
        info::query(driver.clone(), by).await?.send_keys(keys).await?;
    
        Ok(())
    }
    
    pub async fn send_keys_from(driver: WebDriver, by: By, index: usize, keys: &str, timeout: u64) -> WebDriverResult<()> {
        info::query_all(driver.clone(), by, timeout).await?.get(index).unwrap().send_keys(keys).await?;
    
        Ok(())
    }
}