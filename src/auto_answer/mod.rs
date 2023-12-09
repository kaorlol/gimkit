use std::time::Duration;
use serde_json::Value;
use thirtyfour::prelude::*;
use tokio::time::sleep;
use colored::Colorize;
use crate::{document::*, util::*};

async fn check_completed(driver: &WebDriver) -> WebDriverResult<bool> {
    let selector = By::Css("div[class='sc-kmbxGf jjrWnM']");
    if let Err(_) = info::find(driver, &selector).await {
        return Ok(false);
    }

    println!("{}", "\nAssignment completed".blue());

    Ok(true)
}

async fn interferance(driver: &WebDriver) -> WebDriverResult<()> {
    let selector = By::Css("button[class='ant-btn css-1k9m51z ant-btn-primary ant-btn-lg']");
    if let Err(_) = info::find(driver, &selector).await {
        return Ok(());
    }

    println!("{}", "\nInterferance detected".blue());
    actions::click_from(driver, &selector, 1, 10).await?;
    println!("{}", "Interferance removed".blue());

    let selector = By::Css("div[class='sc-hdWpuu cCeQmZ']");
    actions::click(driver, &selector).await?;

    Ok(())
}

async fn get(driver: &WebDriver, data: &Value) -> WebDriverResult<(String, String)> {
    let selector = By::Css("span[class='notranslate lang-en']");
    let question = info::get_text(driver, &selector).await?;

    if let Some(entry) = data.as_array().and_then(|arr| arr.iter().find(|entry| {
        entry["question"].as_str() == Some(question.as_str())
    })) {
        if let Some(answer) = entry["answers"].as_array().and_then(|arr| arr.first()) {
            let answer_string = answer.to_string().trim_matches('"').to_owned();
            return Ok((answer_string, question));
        }
    }

    Ok((String::new(), String::new()))
}

#[async_recursion::async_recursion]
pub async fn auto_answer(driver: &WebDriver, answers: &Value) -> WebDriverResult<()> {
    if check_completed(driver).await? {
        return Ok(());
    }

    interferance(driver).await?;

    let (answer, question) = get(driver, answers).await?;
    if answer != "" {
        let selector = By::XPath(&format!("//span[text()='{}']", answer));
        let is_multi = info::exists(driver, &selector).await?;
        if is_multi {
            actions::click(driver, &selector).await?;
        } else {
            let selector = By::Css("input[placeholder='Enter answer here...']");
            actions::send_keys(driver, &selector, &answer).await?;
            sleep(Duration::from_secs_f64(0.25)).await;
            let selector = By::Css("div.sc-EPlqQ.iwsjJJ");
            actions::click(driver, &selector).await?;
        }

        sleep(Duration::from_secs_f64(0.25)).await;

        let selector = By::Css("div.sc-jvfpSw.eWPjkh");
        let elements = info::query_all(driver, &selector, 1).await?;
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