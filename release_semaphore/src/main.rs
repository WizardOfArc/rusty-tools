use std::collections::HashMap;
use std::fs::read_to_string;
use std::env;
use std::collections::HashSet;
use std::io::stdin;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use clap::Parser; 
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;


#[derive(clap::ValueEnum, Clone, Debug)]
enum Command {
    NewRelease,
    UpdateContributors,
    UpdateReleaseTag,
    UpdateNewSites,
    UpdateSlackMapping,
    UpdateDeleteSites,
    UpdateConfigs,
    ShowState,
    PromptMergeState,
    NotifyOnStage,
    ReleaseComplete,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    command: Command,
}

#[derive(Serialize, Deserialize)]
struct SemaphoreState {
    contributors: Vec<String>,
    release_tag: String,
    new_sites: Vec<String>,
    configs_to_update: Vec<String>,
    sites_to_delete: Vec<String>,
}

impl SemaphoreState {
    fn new() -> SemaphoreState {
        SemaphoreState {
            contributors: Vec::new(),
            release_tag: String::new(),
            new_sites: Vec::new(),
            configs_to_update: Vec::new(),
            sites_to_delete: Vec::new(),
        }
    }

    fn from_json_string(json_string: &str) -> Result<SemaphoreState> {
        serde_json::from_str(json_string)
    }

    fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self)
    }

    fn save(&self) {
        let directory = env::var("SEMAPHORE_SUPPORT_DIR").unwrap();
        let file_path = format!("{}/semaphore_state.json", directory);
        std::fs::write(file_path, self.to_json_string().unwrap()).unwrap()
    }

    fn show_state(&self) {
        println!("Release Tag: {:?}", self.release_tag);
        println!("New Sites: {:?}", self.new_sites.join(" "));
        println!("Configs to Update: {:?}", self.configs_to_update.join(" "));
        println!("Sites to Delete: {:?}", self.sites_to_delete.join(" "));
    }
}


fn update_configs(state: &mut SemaphoreState) {
    let mut proceed: String = String::new();
    loop {
        println!("Enter config name ('none' if there are none 'all' for all) otherwise one domain at a time ('q' to end)");
        stdin().read_line(&mut proceed).expect("Failed to read line");
        let config = proceed.trim();
        match config {
            "q" => break,
            "none" => {
                state.configs_to_update.clear();
                break
            },
            "all" => {
                state.configs_to_update.clear();
                state.configs_to_update.push("all".to_string());
                break
            }
            _ => state.configs_to_update.push(config.to_string()),
        }
    }
}

fn update_new_domains(state: &mut SemaphoreState) {
    state.new_sites.clear();
    let mut domain_list_string: String = String::new();
    println!("Enter domains as comma separated list - no space");
    stdin().read_line(&mut domain_list_string).expect("Failed to read line");
    let domains: Vec<String> = domain_list_string.split(",").map(|s| s.trim().to_string()).collect();
    domains.iter().for_each(|domain| {
        state.new_sites.push(domain.to_owned());
    });
}

fn update_delete_domains(state: &mut SemaphoreState) {
    state.sites_to_delete.clear();
    let mut domain_list_string: String = String::new();
    println!("Enter domains to delete (comma separated list)");
    stdin().read_line(&mut domain_list_string).expect("Failed to read line");
    let domains: Vec<String> = domain_list_string.split(",").map(|s| s.trim().to_string()).collect();
    domains.iter().for_each(|domain| {
        state.sites_to_delete.push(domain.to_string());
    });
}

fn contributor_set_from_string(contributor_string: String, slack_id_mapping: &HashMap<String, String>) -> HashSet<String> {
    let contributors: Vec<String> = contributor_string.split(",").map(|s| s.to_string()).collect();
    let mut contributor_set: HashSet<String> = HashSet::new();
    contributors.iter().for_each(|contributor| {
        let new_contributor = contributor.trim().to_string();
        if slack_id_mapping.contains_key(&new_contributor) {
            contributor_set.insert(slack_id_mapping.get(&new_contributor).unwrap().to_string());
        } else {
            println!("No slack id mapping for {}", contributor);
        }
    });
    contributor_set
}

fn update_contributors(state: &mut SemaphoreState, slack_id_mapping: &HashMap<String, String>) {
    state.contributors.clear();
    println!("Enter contributors (comma separated with no spaces)");
    let mut new_contributor_list: String = String::new();
    stdin().read_line(&mut new_contributor_list).expect("Failed to read line");
    let new_contributors = contributor_set_from_string(new_contributor_list, slack_id_mapping);
    new_contributors.iter().for_each(|contributor| {
        println!(" - {}", contributor);
        state.contributors.push(contributor.to_string());
    });
}


fn load_slack_id_mapping(file_path: &str) -> HashMap<String, String> {
    let file_contents = read_to_string(file_path).unwrap();
    let mut slack_id_mapping: HashMap<String, String> = HashMap::new();
    for line in file_contents.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        if parts.len() == 2 {
            slack_id_mapping.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    slack_id_mapping
}

fn save_slack_id_mapping(file_path: &str, slack_id_mapping: &HashMap<String, String>) {
    let mut file_contents: String = String::new();
    for (key, value) in slack_id_mapping {
        file_contents.push_str(&format!("{}:{}\n", key, value));
    }
    std::fs::write(file_path, file_contents).unwrap();
}

fn update_slack_id_mapping(file_path: &str, slack_id_mapping: &mut HashMap<String, String>) {
    let mut slack_ids_to_update: String = String::new();
    println!("Enter slack id mappings (comma separated list spaces only allowed within slack id)");
    println!("example: 'cantcatchme:@Gingerbread Man,jfrost:@Jack Frost'");
    stdin().read_line(&mut slack_ids_to_update).expect("Failed to read line");
    let new_mapping_pairs: Vec<String> = slack_ids_to_update.split(",").map(|s| s.trim().to_string()).collect();
    new_mapping_pairs.iter().for_each(|pair| {
        let parts: Vec<String> = pair.split(":").map(|s| s.trim().to_string()).collect();
        if parts.len() == 2 {
            if parts[1].starts_with("@") {
                slack_id_mapping.insert(parts[0].to_string(), parts[1].to_string());
            } else {
                println!("Invalid slack id format: {}", parts[1]);
            }    
        }
    });
    save_slack_id_mapping(file_path, slack_id_mapping);
}

fn update_release_tag(state: &mut SemaphoreState) {
    let mut release_tag: String = String::new();
    println!("Enter release tag");
    stdin().read_line(&mut release_tag).expect("Failed to read line");
    state.release_tag = release_tag.trim().to_string();
}


fn prompt_merge_state(state: &SemaphoreState) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    
    let mut lines: Vec<String> = Vec::new();

    let _ = &state.contributors.iter().for_each(|contributor| {
        lines.push(format!("{}", contributor));
    });

    lines.push("\nWhat is your merge status?".to_string());
    lines.push(":sonic-wait: (in merge queue)".to_string());
    lines.push(":github-merged-pr: (merged)".to_string());
    lines.push(":tuzki-give-up: (I'm pulling out of release)".to_string());
    let result = lines.join("\n");
    ctx.set_contents(result).unwrap();
    println!("Copied to clipboard - paste in Slack");
}

fn notify_on_stage(state: &SemaphoreState) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut lines: Vec<String> = Vec::new();
    let _ = &state.contributors.iter().for_each(|contributor| {
        lines.push(format!("{}", contributor));
    });
    lines.push("\nYour code is on stage - how does it look?".to_string());
    lines.push(":eyes: (I'm looking now)".to_string());
    lines.push(":thumbsup: (all good)".to_string());
    lines.push(":dont-ship-it: (I've got to revert)".to_string());
    let result = lines.join("\n");
    ctx.set_contents(result).unwrap();
    println!("Copied to clipboard - paste in Slack");
}

fn notify_release_complete(state: &SemaphoreState) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut lines: Vec<String> = Vec::new();
    let _ = &state.contributors.iter().for_each(|contributor| {
        lines.push(format!("{}", contributor));
    });
    if state.configs_to_update.len() > 0 {
        lines.push("\nConfigs to out".to_string());
    } else {
        lines.push("\nNo configs to update".to_string());
    }
    lines.push("\nRelease is complete - please confirm".to_string());
    let result = lines.join("\n");
    ctx.set_contents(result).unwrap();
    println!("Copied to clipboard - paste in Slack");
}

fn prep_for_new_release(state: &mut SemaphoreState) {
    println!("Prepping for new release");
    println!(" - clearing old data...");
    state.contributors.clear();
    state.release_tag.clear();
    state.new_sites.clear();
    state.configs_to_update.clear();
    state.sites_to_delete.clear();
    println!(" - old data cleared.\n please run udpate-configs, update-new-sites, update-delete-sites, update-release-tag, update-contributors");
}

fn main() {
    let args: Args = Args::parse();
    let directory_env_var = "SEMAPHORE_SUPPORT_DIR";
    let directory = match env::var(directory_env_var) {
        Ok(val) => val,
        Err(_) => {
            println!("Please set the {} environment variable to an existing directory", directory_env_var);
            std::process::exit(1);
        }
    };
    if !Path::new(&directory).exists() {
        println!("{} does not exist, please set {} env var to an existing directory", directory, directory_env_var);
        std::process::exit(1);
    }
    if !Path::new(&directory).is_dir() {
        println!("{} is not a directory, please set the env var {} to a directory", directory, directory_env_var);
        std::process::exit(1);
    }
    let slack_id_mapping_file = format!("{}/slack_id_mapping.txt", directory);
    let semaphore_state_file = format!("{}/semaphore_state.json", directory);
    let mut semaphore_state: SemaphoreState;
    let mut slack_id_mapping: HashMap<String, String>;

    if Path::new(&semaphore_state_file).exists() {
        let state_file_contents = read_to_string(&semaphore_state_file).unwrap();
        semaphore_state = SemaphoreState::from_json_string(&state_file_contents).unwrap();
    } else {
        println!("State file does not exist");
        semaphore_state = SemaphoreState::new();
        println!("Creating new state file with json contents: {}", semaphore_state.to_json_string().unwrap());
    }

    if Path::new(&slack_id_mapping_file).exists() {
        slack_id_mapping = load_slack_id_mapping(&slack_id_mapping_file);
    } else {
        println!("Slack mapping file does not exist");
        slack_id_mapping = HashMap::new();
        println!("Please create a slack mapping file by running 'update-slack-mapping'");
    }
    match args.command {
        Command::UpdateContributors => {
            update_contributors(&mut semaphore_state, &slack_id_mapping);
            semaphore_state.save();
        }
        Command::UpdateReleaseTag => {
            update_release_tag(&mut semaphore_state);
            semaphore_state.save();
        }
        Command::UpdateNewSites => {
            update_new_domains(&mut semaphore_state);
            semaphore_state.save();
        }
        Command::UpdateDeleteSites => {
            update_delete_domains(&mut semaphore_state);
            semaphore_state.save();
        }
        Command::UpdateConfigs => {
            update_configs(&mut semaphore_state);
            semaphore_state.save();
        }
        Command::UpdateSlackMapping => {
            update_slack_id_mapping(&slack_id_mapping_file, &mut slack_id_mapping);
        }
        Command::NotifyOnStage => {
            notify_on_stage(&mut semaphore_state);
        }
        Command::PromptMergeState => {
            prompt_merge_state(&semaphore_state);
        }
        Command::NewRelease => {
            prep_for_new_release(&mut semaphore_state);
            semaphore_state.save();
        }
        Command::ShowState => {
            semaphore_state.show_state();
        }
        Command::ReleaseComplete => {
            notify_release_complete(&semaphore_state);
        }
    }

}
