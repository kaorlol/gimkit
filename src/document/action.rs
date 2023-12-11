use crate::document::info::{query, query_all};
use thirtyfour::prelude::*;

pub async fn click(driver: &WebDriver, by: &By) -> WebDriverResult<()> {
	let element = &query(driver, by).await?;
	element.click().await?;

	Ok(())
}

pub async fn click_from(driver: &WebDriver, by: &By, index: usize) -> WebDriverResult<()> {
	let elements = query_all(driver, by).await?;
	let element = elements.get(index).ok_or_else(|| {
		WebDriverError::NoSuchElement(format!(
			"No element found with index {:?}, found elements: {:?}",
			index, elements
		))
	})?;
	element.click().await?;

	Ok(())
}

pub async fn send_keys(driver: &WebDriver, by: &By, keys: &str) -> WebDriverResult<()> {
	let elements = &query(driver, by).await?;
	elements.send_keys(keys).await?;

	Ok(())
}

pub async fn send_keys_from(
	driver: &WebDriver,
	by: &By,
	index: usize,
	keys: &str,
) -> WebDriverResult<()> {
	let elements = query_all(driver, by).await?;
	let element = elements.get(index).unwrap();
	element.send_keys(keys).await?;

	Ok(())
}
