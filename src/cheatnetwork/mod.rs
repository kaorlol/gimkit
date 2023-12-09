use std::time::Duration;
use colored::Colorize;
use serde_json::{Value, json};
use thirtyfour::prelude::*;
use tokio::time::{sleep, Instant};
use crate::document::*;

async fn login(driver: WebDriver, old_handle: WindowHandle, data: Value) -> WebDriverResult<()> {
    let new_handle =driver.new_tab().await?;
    driver.switch_to_window(new_handle.clone()).await?;
    driver.goto("https://cheatnetwork.eu/login").await?;

    actions::send_keys(driver.clone(), By::Tag("input"), data["backup-key"].as_str().unwrap()).await?;
    actions::click_from(driver.clone(), By::Tag("button"), 2, 10).await?;

    driver.close_window().await?;
    driver.switch_to_window(old_handle.clone()).await?;

    Ok(())
}

async fn get_grabber(driver: WebDriver, old_handle: WindowHandle) -> WebDriverResult<String> {
    let new_handle = driver.new_tab().await?;
    driver.switch_to_window(new_handle.clone()).await?;
    driver.goto("https://cheatnetwork.eu/services/gimkit").await?;

    let selector = By::Tag("textarea");
    let grabber = info::recursion::get_value(driver.clone(), selector.clone(), "Loading...").await?;

    driver.close_window().await?;
    driver.switch_to_window(old_handle.clone()).await?;

    Ok(grabber)
}

pub async fn get_answers(driver: WebDriver, old_handle: WindowHandle, data: Value) -> WebDriverResult<Value> {
    let start = Instant::now();
    println!("{}", "\nGetting answers...".blue());

    login(driver.clone(), old_handle.clone(), data.clone()).await?;

    let grabber = get_grabber(driver.clone(), old_handle.clone()).await?;

    driver.execute(&grabber, vec![]).await?;

    sleep(Duration::from_secs(1)).await;

    let new_handle = driver.windows().await?.last().unwrap().clone();
    driver.switch_to_window(new_handle.clone()).await?;

    let answers = info::query_all(driver.clone(), By::Css(".question-box"), 10).await?;
    let mut answers_text = Vec::new();
    for answer in answers {
        let question = answer.query(By::Tag("h2")).first().await?.text().await?;
        let answer_elements = answer.query(By::Tag("li")).all().await?;
        let mut answers = Vec::new();
        for answer_element in answer_elements {
            answers.push(answer_element.text().await?);
        }

        answers_text.push(json!({
            "question": question,
            "answers": answers
        }));
    }

    driver.close_window().await?;
    driver.switch_to_window(old_handle.clone()).await?;

    println!("{} {}", "Answers retrieved".blue(), format!("(took {}s)", start.elapsed().as_secs()).blue());

    Ok(serde_json::to_value(answers_text)?)
}