use crate::{
	document::{action, info},
	utils::get_random_number,
};

use async_recursion::async_recursion;
use colored::Colorize;
use serde_json::Value;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio::time::sleep;

async fn check_completed(driver: &WebDriver) -> WebDriverResult<bool> {
	let completed_text = By::Css("div.sc-kmbxGf.jjrWnM");
	if let Err(_) = info::find(driver, &completed_text).await {
		return Ok(false);
	}

	println!("{}", "\nAssignment completed".blue());

	Ok(true)
}

async fn interference(driver: &WebDriver) -> WebDriverResult<()> {
	let close_button = By::Css("button.ant-btn.css-1k9m51z.ant-btn-primary.ant-btn-lg");
	if let Err(_) = info::find(driver, &close_button).await {
		return Ok(());
	}

	println!("{}", "\nInterference detected".blue());
	action::click_from(driver, &close_button, 1).await?;
	println!("{}", "Interference removed".blue());

	let start_button = By::Css("div.sc-hdWpuu.cCeQmZ");
	action::click(driver, &start_button).await?;

	Ok(())
}

async fn get(driver: &WebDriver, data: &Value) -> WebDriverResult<(String, String)> {
	let question_selector = By::Css("span.notranslate.lang-en");
	let question = info::get_text(driver, &question_selector).await?;

	if let Some(entry) = data.as_array().and_then(|arr| {
		arr.iter()
			.find(|entry| entry["question"].as_str() == Some(question.as_str()))
	}) {
		if let Some(answer) = entry["answers"].as_array().and_then(|arr| arr.first()) {
			let answer_string = answer.to_string().trim_matches('"').to_owned();
			return Ok((answer_string, question));
		}
	}

	Ok((String::new(), String::new()))
}

pub async fn start_assignment(driver: &WebDriver) -> WebDriverResult<()> {
	println!("{}", "\nStarting assignment...".blue());
	let loading_icon = By::Tag("svg");
	info::query(driver, &loading_icon).await?;

	let start_button = By::Css(".btn-pushable");
	action::click(driver, &start_button).await?;

	println!("{}", "Assignment started\nLoading assignment...".blue());
	let start_button = By::Css("div.sc-hdWpuu.cCeQmZ");
	action::click(driver, &start_button).await?;

	sleep(Duration::from_secs(1)).await;

	Ok(())
}

#[async_recursion]
pub async fn auto_answer(driver: &WebDriver, answers: &Value) -> WebDriverResult<()> {
	if check_completed(driver).await? {
		return Ok(());
	}

	interference(driver).await?;

	let (answer, question) = get(driver, answers).await?;
	if answer != "" {
		let answer_text = By::XPath(&format!("//span[text()=\"{}\"]", answer));
		let is_multi = info::exists(driver, &answer_text).await?;
		if is_multi {
			action::click(driver, &answer_text).await?;
		} else {
			let text_box = By::Css("input[placeholder='Enter answer here...']");
			action::send_keys(driver, &text_box, &answer).await?;

			sleep(Duration::from_secs_f64(0.25)).await;

			let submit_button = By::Css("div.sc-EPlqQ.iwsjJJ");
			action::click(driver, &submit_button).await?;
		}

		sleep(Duration::from_secs_f64(0.25)).await;

		let buttons = By::Css("div.sc-jvfpSw.eWPjkh");
		let elements = info::query_all(driver, &buttons).await?;
		if let Some(element) = elements.get(2) {
			element.click().await?;

			let wait_time = get_random_number(3..10);
			println!("{}", format!("\nQuestion: '{question}', Answer Submitted: '{answer}'\nWaiting for {wait_time} seconds").blue());

			sleep(Duration::from_secs(wait_time)).await;
		}
	}

	auto_answer(driver, answers).await?;

	Ok(())
}
