use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

use chrono::prelude::*;
use clap::Parser;
use slack_hook::{PayloadBuilder, Slack};

const WEBHOOK_KEY: &str = "SEARCH_SLACK_WEBHOOK";
const BOT_NAME: &str = "Search Sites Team Update Bot";
const ICON_EMOJI: &str = ":robot_face:";
const WORKFLOW_LINK: &str = "Click the link to start the workflow:\nhttps://slack.com/shortcuts/Ft0816961QNQ/ec60e56ad0f54f3025cc1d5ef4e6d53f\n";
const CHANNEL_KEY: &str = "SEARCH_SITES_TEAM_CHANNEL";
type SlackResult<T> = std::result::Result<T, SlackError>;

#[derive(Debug)]
enum SlackError {
    MissingWebhook,
    UnableToMakeSlackClient,
    UnableToBuildPayload,
    UnableToSendMessage,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    name: String,
}

fn send_to_slack(message: &str, user: &str) -> SlackResult<()> {
    let webhook_url = env::var(WEBHOOK_KEY).map_err(|_| SlackError::MissingWebhook)?;
    let slack =
        Slack::new(webhook_url.as_str()).map_err(|_| SlackError::UnableToMakeSlackClient)?;
    let payload = PayloadBuilder::new()
        .text(message)
        .username(BOT_NAME)
        .channel(user)
        .icon_emoji(ICON_EMOJI)
        .build()
        .map_err(|_| SlackError::UnableToBuildPayload)?;
    slack
        .send(&payload)
        .map_err(|_| SlackError::UnableToSendMessage)
}

fn get_day_for_today() -> Weekday {
    let now: DateTime<Local> = Local::now();
    now.weekday()
}

fn lookup_slack_id(name: &str) -> Option<String> {
    if name == "team" {
        let slack_channel = env::var(CHANNEL_KEY).ok()?;
        return Some(slack_channel.to_string());
    }
    let mut mapping: HashMap<String, String> = HashMap::new();
    let path = env::var("SLACK_ID_MAPPING").ok()?;
    let file_contents = read_to_string(path).ok()?;
    for line in file_contents.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        if parts.len() == 2 {
            mapping.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    let lower_name = name.to_lowercase();
    mapping.get(&lower_name).map(|s| s.to_string())
}

fn build_message(name: String, day: Weekday) -> String {
    match day {
        Weekday::Mon => format!(
            "Happy Monday {}!\n\
  I hope you had a great weekend.\n\
  Would you kindly send me your update for last week please? :pls:\n\
  {}\
  I'd really appreciate it.\n\n\
  Thank you!\n{}",
            name, WORKFLOW_LINK, BOT_NAME
        ),
        Weekday::Tue => format!(
            "Happy Tuesday {}!\n\
  I hope you had a great long weekend.\n\
  Would you kindly send me your update for last week please? :pls:\n\
  {}\
  I'd really appreciate it.\n\n\
  Thank you!\n{}",
            name, WORKFLOW_LINK, BOT_NAME
        ),
        Weekday::Wed => format!(
            "Hey {}!\n\
  I hope you had a good Wednesday so far, I know it's the middle of the week.\n\
  I'm either really early (like Thanksgiving) or really late.\n\
  Would you kindly send me your update please? :pls:\n\
  {}\
  I'd really appreciate it.\n\n\
  Thank you!\n{}",
            name, WORKFLOW_LINK, BOT_NAME
        ),
        Weekday::Thu => format!(
            "Hey {}!\n\
  I hope you had a good Thursday so far, and have fun, long weekend plans.\n\
  Would you give me your update for the week so I can get it done early on Monday?\n\
  {}\
  I'd really appreciate it.\n\n\
  Thank you!\n{}",
            name, WORKFLOW_LINK, BOT_NAME
        ),
        Weekday::Fri => format!(
            "Hey {}!\n\
  I hope you had a good Friday so far, and have fun weekend plans.\n\
  Would you give me your update for the week so I can get it done early on Monday?\n\
  {}\
  I'd really appreciate it.\n\n\
  Thank you!\n{}",
            name, WORKFLOW_LINK, BOT_NAME
        ),
        Weekday::Sat => format!(
            "Hey {}!\n\
  I forgot to ping you on Friday\n\
  Would you give me your update for the week so I can get it done early on Monday?\n\
  I'd really appreciate it.
  It's the weekend though - so ignore this until Monday\n\n\
  Thank you!\n{}",
            name, BOT_NAME
        ),
        Weekday::Sun => format!(
            "Hey {}!\n\
  Ignore this until Monday...  it's still the weekend.\n\n\
  Thank you!\n{}",
            name, BOT_NAME
        ),
    }
}

fn main() {
    let args = Args::parse();
    match lookup_slack_id(&args.name) {
        None => println!("User not found"),
        Some(user) => {
            let weekday: Weekday = get_day_for_today();
            let message = build_message(args.name, weekday);
            let _ = send_to_slack(&message, &user);
        }
    }
}
