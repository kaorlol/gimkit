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
	let completed_text = By::XPath("//div[text()=\"Complete!\"]");
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

	let start_button = By::Css("div.sc-bVhkf.debiKD");
	action::click(driver, &start_button).await?;

	Ok(())
}

async fn get_answer(driver: &WebDriver) -> WebDriverResult<String> {
	let kit_selector = By::Id("kit");
	let kit_element = info::find(driver, &kit_selector).await?;
	let data_string = match kit_element.attr("data").await {
		Ok(data) => data.unwrap(),
		Err(_) => return Ok("couldn't find kit".to_string()),
	};

	let data: Value = serde_json::from_str(&data_string).unwrap();
	let questions = data["kit"]["questions"].as_array().unwrap();

	let question_selector = By::Css("span.notranslate.lang-en");
	let question_ = info::get_text(driver, &question_selector).await?;
	let correct_answer = questions
		.iter()
		.find_map(|question| {
			let text = question["text"].as_str().unwrap_or_default();
			if text == question_ {
				let answers = &question["answers"];
				for answer in answers.as_array().unwrap() {
					if answer["correct"].as_bool().unwrap_or(false) {
						return Some(answer["text"].as_str().unwrap_or_default().to_string());
					}
				}
			}
			None
		})
		.unwrap_or_else(|| "Answer not found".to_string());

	Ok(correct_answer)
}

pub async fn start_assignment(driver: &WebDriver) -> WebDriverResult<()> {
	println!("{}", "\nStarting assignment...".blue());
	let loading_icon = By::Tag("svg");
	info::query(driver, &loading_icon).await?;

	let start_button = By::Css(".btn-pushable");
	action::click(driver, &start_button).await?;

	println!("{}", "Assignment started\nLoading assignment...".blue());
	let start_button = By::Css("div.sc-bVhkf.debiKD");
	action::click(driver, &start_button).await?;

	sleep(Duration::from_secs(1)).await;

	Ok(())
}

#[async_recursion]
pub async fn auto_answer(driver: &WebDriver) -> WebDriverResult<()> {
	if check_completed(driver).await? {
		return Ok(());
	}

	interference(driver).await?;

	let answer = get_answer(driver).await?;
	if answer != "Answer not found" {
		let answer_text = By::XPath(&format!("//span[text()=\"{}\"]", answer));
		let is_multi = info::exists(driver, &answer_text).await?;
		if is_multi {
			action::click(driver, &answer_text).await?;
		} else {
			let text_box = By::Css("input[placeholder='Enter answer here...']");
			action::send_keys(driver, &text_box, &(answer.clone() + Key::Enter)).await?;
		}

		sleep(Duration::from_secs_f64(0.25)).await;

		let continue_selector = By::XPath("//div[text()=\"Continue\"]");
		let continue_button = info::find(driver, &continue_selector).await;

		if continue_button.is_ok() {
			continue_button?.click().await?;

			let wait_time = get_random_number(3..10);
			// Question: '{question}',
			println!(
				"{}",
				format!("\nAnswer Submitted: '{answer}'\nWaiting for {wait_time} seconds").blue()
			);

			sleep(Duration::from_secs(wait_time)).await;
		}
	}

	auto_answer(driver).await?;

	Ok(())
}
