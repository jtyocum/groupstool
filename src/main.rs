use clap::{Parser, Subcommand};
use curl::easy::Easy;
use regex::Regex;
use serde::Deserialize;
use std::error::Error;

const GROUPS_API: &'static str = env!("GROUPS_API");

#[derive(Parser)]
#[clap(name = "groupstool")]
#[clap(about = "UW Groups CLI", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a user's group memberships
    #[clap(arg_required_else_help = true)]
    GroupsByMember {
        /// Certificate + Key (used to authenticate with the web service)
        auth_cert: String,
        /// NetID (typical personal NetID is 8 characters or less)
        #[clap(validator = is_valid_netid)]
        member_uid: String,
    },
    /// List group members
    #[clap(arg_required_else_help = true)]
    ListMembers {
        /// Certificate + Key (used to authenticate with the web service)
        auth_cert: String,
        /// Group ID (typically starts with u_ or uw_)
        #[clap(validator = is_valid_group)]
        group_id: String,
    },
    /// Add a user to a group
    #[clap(arg_required_else_help = true)]
    AddMember {
        /// Certificate + Key (used to authenticate with the web service)
        auth_cert: String,
        /// Group ID (typically starts with u_ or uw_)
        #[clap(validator = is_valid_group)]
        group_id: String,
        /// NetID (typical personal NetID is 8 characters or less)
        #[clap(validator = is_valid_netid)]
        member_uid: String,
    },
    /// Remove a user from a group
    #[clap(arg_required_else_help = true)]
    RemoveMember {
        /// Certificate + Key (used to authenticate with the web service)
        auth_cert: String,
        /// Group ID (typically starts with u_ or uw_)
        #[clap(validator = is_valid_group)]
        group_id: String,
        /// NetID (typical personal NetID is 8 characters or less)
        #[clap(validator = is_valid_netid)]
        member_uid: String,
    },
}

fn is_valid_netid(s: &str) -> Result<(), String> {
    let re = Regex::new(r"^[a-z][a-z0-9]{0,7}$").unwrap();
    if re.is_match(s) {
        Ok(())
    } else {
        Err(format!(
            "NetID may be too long or contain invalid characters."
        ))
    }
}

fn is_valid_group(s: &str) -> Result<(), String> {
    let re = Regex::new(r"^(u|uw)_[a-z0-9]+[a-z0-9_\-]*$").unwrap();
    if re.is_match(s) {
        Ok(())
    } else {
        Err(format!(
            "Incomplete group ID or contains invalid characters."
        ))
    }
}

fn groups_by_member(auth_cert: &str, member_uid: &str) -> Result<(), Box<dyn Error>> {
    #[derive(Deserialize)]
    struct Response {
        data: Vec<Data>,
    }

    #[derive(Deserialize)]
    struct Data {
        id: String,
    }

    let mut raw_data = String::new();
    let mut handle = Easy::new();

    handle.ssl_cert(auth_cert)?;
    handle.get(true)?;
    handle.url(&(GROUPS_API.to_owned() + "/search?member=" + member_uid))?;
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            raw_data.push_str(std::str::from_utf8(new_data).unwrap());
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }

    let resp: Response = serde_json::from_str(&raw_data)?;

    for data in resp.data {
        println!("{}", data.id);
    }

    Ok(())
}

fn list_members(auth_cert: &str, group_id: &str) -> Result<(), Box<dyn Error>> {
    #[derive(Deserialize)]
    struct Response {
        data: Vec<Data>,
    }

    #[derive(Deserialize)]
    struct Data {
        id: String,
    }

    let mut raw_data = String::new();
    let mut handle = Easy::new();

    handle.ssl_cert(auth_cert)?;
    handle.get(true)?;
    handle.url(&(GROUPS_API.to_owned() + "/group/" + group_id + "/member"))?;
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            raw_data.push_str(std::str::from_utf8(new_data).unwrap());
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }

    let resp: Response = serde_json::from_str(&raw_data)?;

    for data in resp.data {
        println!("{}", data.id);
    }

    Ok(())
}

fn add_member(auth_cert: &str, group_id: &str, member_uid: &str) -> Result<(), Box<dyn Error>> {
    let mut handle = Easy::new();

    handle.ssl_cert(auth_cert)?;
    handle.put(true)?;
    handle.url(&(GROUPS_API.to_owned() + "/group/" + group_id + "/member/" + member_uid))?;

    handle.perform()?;
    println!(
        "Adding member {} to group {}: {}",
        member_uid,
        group_id,
        handle.response_code()?
    );

    Ok(())
}

fn remove_member(auth_cert: &str, group_id: &str, member_uid: &str) -> Result<(), Box<dyn Error>> {
    let mut handle = Easy::new();

    handle.ssl_cert(auth_cert)?;
    handle.custom_request("DELETE")?;
    handle.url(&(GROUPS_API.to_owned() + "/group/" + group_id + "/member/" + member_uid))?;

    handle.perform()?;
    println!(
        "Removing member {} from group {}: {}",
        member_uid,
        group_id,
        handle.response_code()?
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Cli::parse();

    match &args.command {
        Commands::GroupsByMember {
            auth_cert,
            member_uid,
        } => {
            groups_by_member(auth_cert, member_uid)?;
        }
        Commands::ListMembers {
            auth_cert,
            group_id,
        } => {
            list_members(auth_cert, group_id)?;
        }
        Commands::AddMember {
            auth_cert,
            group_id,
            member_uid,
        } => {
            add_member(auth_cert, group_id, member_uid)?;
        }
        Commands::RemoveMember {
            auth_cert,
            group_id,
            member_uid,
        } => {
            remove_member(auth_cert, group_id, member_uid)?;
        }
    }

    Ok(())
}
