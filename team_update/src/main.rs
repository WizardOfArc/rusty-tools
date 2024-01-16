use clap::Parser;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use chrono::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    name: String,
}

fn get_day_for_today() -> Weekday {
    let now: DateTime<Local> = Local::now();
    now.weekday()

}

fn build_message(name: String, day: Weekday) -> String {
    match day {
        Weekday::Mon => format!("Happy Monday {}!\n\
  I hope you had a great weekend.\n\
  Would you kindly send me your update for last week please? :pls:\n\
  I'd really appreciate it.\n\n\
  Thank you!", name),
        Weekday::Tue => format!("Happy Tuesday {}!\n\
  I hope you had a great long weekend.\n\
  Would you kindly send me your update for last week please? :pls:\n\
  I'd really appreciate it.\n\n\
  Thank you!", name),
        Weekday::Wed => format!("Hey {}!\n\
  I hope you had a good Wednesday so far, I know it's the middle of the week.\n\
  I'm either really early (like Thanksgiving) or really late.\n\
  Would you kindly send me your update please? :pls:\n\
  I'd really appreciate it.\n\n\
  Thank you!", name),
        Weekday::Thu => format!("Hey {}!\n\
  I hope you had a good Thursday so far, and have fun, long weekend plans.\n\
  Would you give me your update for the week so I can get it done early on Monday?\n\
  I'd really appreciate it.\n\n\
  Thank you!", name),
        Weekday::Fri => format!("Hey {}!\n\
  I hope you had a good Friday so far, and have fun weekend plans.\n\
  Would you give me your update for the week so I can get it done early on Monday?\n\
  I'd really appreciate it.\n\n\
  Thank you!", name),
        Weekday::Sat => format!("Hey {}!\n\
  I forgot to ping you on Friday\n\
  Would you give me your update for the week so I can get it done early on Monday?\n\
  I'd really appreciate it.
  It's the weekend though - so ignore this until Monday\n\n\
  Thank you!", name),
        Weekday::Sun => format!("Hey {}!\n\
  Ignore this until Monday...  it's still the weekend.\n\n\
  Thank you!", name)
    }
}

fn main() {
    let args = Args::parse();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let weekday: Weekday = get_day_for_today();
    ctx.set_contents(build_message(args.name, weekday)).unwrap();
}
