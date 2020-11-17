use reqwest;
use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Repository {
    pub author: String,
    pub name: String,
    pub avatar: String,
    pub url: String,
    pub description: String,
    pub language: String,
    //pub language_color: String,
    pub stars: String,
    pub forks: String,
    //pub current_period_stars: String,
    //pub built_by: Vec<Person>,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Person {
    pub href: String,
    pub avatar: String,
    pub username: String,
}
#[derive(Serialize, Deserialize, PartialEq)]
pub struct ProgrammingLanguage {
    pub id: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, PartialEq)]
pub struct SpokenLanguage {
    pub url_param: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Developer {
    pub username: String,
    pub name: String,
    pub ttype: String,
    pub url: String,
    pub avatar: String,
    pub repo: Repo,
}
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Repo {
    pub name: String,
    pub description: String,
    pub url: String,
}

const URL: &str = "https://hackertab.pupubird.com";

// Fetch trending repositories on GitHub
//
//Parameters:
//  language (str, optional):  Filtering by language, eg: python
//   spoken_language_code (str, optional): The spoken language, eg: en for english
//  since (str, optional): The time range, choose from: [daily, weekly, monthly]. Defaults to "daily"

//Returns:
//    A list of dicts containing information for the trending repositories found
pub async fn fetch_repos(
    language: Option<&str>,
    spoken_lang_code: Option<&str>,
    since: Option<&str>,
) -> Result<Vec<Repository>, Box<dyn Error>> {
    let mut query = String::from("/repositories?");
    if let Some(language) = language {
        if check_language_validity(language).await? {
            query = format!("{}language={}", query, language);
        }
    } else {
        query = format!("{}language=", query);
    }
    if let Some(since) = since {
        // check if since is either "daily", "weekly", or "monthly"
        if check_since_validity(since)? {
            query = format!("{}&since={}", query, since);
        }
    } else {
        // defaults to 'daily'
        query = format!("{}&since=daily", query);
    }
    if let Some(spoken_lang_code) = spoken_lang_code {
        if check_spoken_lang_code_validity(spoken_lang_code).await? {
            //needs formatting
            query = format!("{}&spoken_lang_code={}", query, spoken_lang_code);
        }
    } else {
        query = format!("{}&spoken_lang_code=", query);
    }

    let full_url = format!("{}{}", URL, query);
    println!("here's final url:{}", full_url);
    let res = reqwest::get(&full_url)
        .await?
        .json::<Vec<Repository>>()
        .await?;
    return Ok(res);
}

// Fetch trending devs on GitHub
//
//Paramters:
//  * language
// * since
//
// Returns
// * a Result enum of list of devs or error.
pub async fn fetch_developers(
    language: Option<&str>,
    since: Option<&str>,
) -> Result<Vec<Developer>, Box<dyn std::error::Error>> {
    let mut query = format!("/developers?");
    if let Some(language) = language {
        if check_language_validity(language).await? {
            // need formating before being pushed to the querry
            query = format!("{}language={}", query, language);
        }
    } else {
        query = format!("{}language=", query);
    }

    if let Some(since) = since {
        if check_since_validity(&since)? {
            query = format!("{}&since={}", query, since);
        }
    } else {
        query = format!("{}&since=daily", query);
    }
    let full_url = format!("{}{}", URL, query);
    let res = reqwest::get(&full_url)
        .await?
        .json::<Vec<Developer>>()
        .await?;
    Ok(res)
}

//Check if the language exists.
// parameters:
// langugage(str): The language, eg: python.
//
// Returns:
// A boolena value. True for valid langugae, False otherwise.
async fn check_language_validity(language: &str) -> Result<bool, Box<dyn Error>> {
    let languages: Vec<ProgrammingLanguage> = languages_list().await?;
    let language = language.to_ascii_lowercase();

    for lang in languages {
        if language == lang.name.to_ascii_lowercase() {
            return Ok(true);
        }
    }
    return Ok(false);
}

// Check if the spoken language exits.
//
// Parameters:
// lang_code(str): the spoken language code, eg: en for english
//
// Returns:
// A boolean value. True for valid speoken langauge, False otherwise
async fn check_spoken_lang_code_validity(lang_code: &str) -> Result<bool, Box<dyn Error>> {
    let spoken_languages = spoken_languages_list().await?;
    let lang_code = lang_code.to_ascii_lowercase();

    for lang in spoken_languages {
        if lang_code == lang.name.to_ascii_lowercase() {
            return Ok(true);
        }
    }
    Ok(false)
}
// Check if the time range value is correct
//
// Parameters
// * since(&str): the time range
// Returns
// * a boolean alue. true for valid parameter, false otherwise
fn check_since_validity(since: &str) -> Result<bool, Box<dyn Error>> {
    Ok(["daily", "weekly", "monthly"].contains(&since))
}
// Fetches programming languages from GitHub.
//
// Paramters
// *
//Returns
// *
async fn languages_list() -> Result<Vec<ProgrammingLanguage>, Box<dyn Error>> {
    let url = format!("{}/languages", URL);
    let response: Vec<ProgrammingLanguage> = reqwest::get(&url)
        .await?
        .json::<Vec<ProgrammingLanguage>>()
        .await?;

    Ok(response)
}
// Fetch spoken languages from GitHub.
//
// Returns
// * A Result enum of either vector of spoken languages when successful or error in case of failure
async fn spoken_languages_list() -> Result<Vec<SpokenLanguage>, Box<dyn Error>> {
    let url = format!("{}/spoken_languages?", URL);
    let response: Vec<SpokenLanguage> = reqwest::get(&url)
        .await?
        .json::<Vec<SpokenLanguage>>()
        .await?;
    Ok(response)
}
