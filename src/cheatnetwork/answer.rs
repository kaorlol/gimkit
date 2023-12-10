use crate::{
    cheatnetwork::login,
    document::{info, recursion},
};

use colored::Colorize;
use serde_json::{json, Value};
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Instant};

async fn get_grabber(driver: &WebDriver, old_handle: &WindowHandle) -> WebDriverResult<String> {
    let tab_handle = driver.new_tab().await?;
    driver.switch_to_window(tab_handle).await?;
    driver
        .goto("https://cheatnetwork.eu/services/gimkit")
        .await?;

    let script = By::Tag("textarea");
    let grabber = recursion::get_value(driver, &script, "Loading...").await?;

    driver.close_window().await?;
    driver.switch_to_window(old_handle.clone()).await?;

    Ok(grabber)
}

pub async fn get_answers(
    driver: &WebDriver,
    old_handle: &WindowHandle,
    data: Value,
) -> WebDriverResult<Value> {
    let start = Instant::now();
    println!("{}", "\nGetting answers...".blue());

    login(driver, old_handle, &data).await?;

    let grabber = get_grabber(driver, old_handle).await?;
    driver.execute(&grabber, Vec::new()).await?;

    sleep(Duration::from_secs(1)).await;

    let windows = driver.windows().await?;
    let new_handle = windows.last().unwrap();
    driver.switch_to_window(new_handle.clone()).await?;

    let answers = info::query_all(driver, &By::Css(".question-box")).await?;
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

    let time = format!("(took {:.2}s)", start.elapsed().as_secs_f32());
    println!("{} {}", "Answers retrieved".blue(), time.dimmed());

    Ok(serde_json::to_value(answers_text)?)
}
