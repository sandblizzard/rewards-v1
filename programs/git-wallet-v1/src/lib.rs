use std::io::Cursor;
use std::io::Read;
use std::str::from_utf8;
use byte_order::{LittleEndian, ReadBytesExt};
use serde::{Serialize, Deserialize};

use anchor_lang::prelude::*;
use anchor_lang::{AnchorInstruction, AnchorClient};
// use github_api::{PullRequest, GithubApiClient}; //replced by octocrab - to be implemented

static mut JSON_FILE_DATA: &[u8] = &[]; // instance of json data to be used

// rework this to use domain or one of the other existent structs
#[derive(Serialize, Deserialize)]
struct List {
    name: String,
    profiles: Vec<Profile>,
}

#[derive(Serialize, Deserialize)]
struct Profile {
    github: String,
    solanaAddress: String,
}


pub fn read_json_file(input: &[u8]) -> Vec<u8> {
    //assuming the JSON file is stored in the acconut passed here
    let json_file_account = AnchorInstruction::deserialize(input).unwrap().accounts[0];

    // Read the contents of the JSON file account
    let json_file_data = json_file_account.borrow_data();

    json_file_data
}

// Function to check if the instruction is a valid pull request
// assuming username + address are a data pair in the instruction data
pub fn is_valid_pull_request_insruction(instruction: &AnchorInstruction) -> bool {
    // IF we are sending account + address linked as an instruction
    let (username, pull_request_id) = match instruction.data {
        Ok(data) => {
            let mut cursor = Cursor::new(data);
            let username_len = cursor.read_u8().unwrap() as usize;
            let username = cursor.get_ref()[..username_len]
                .to_vec();
            cursor.set_position(username_len as u64);
            let pull_request_id = cursor.read_u8().unwrap();
            (username, pull_request_id)
        },
        Err(_) => return false,
    };
    true

    // Use the Github API to get the pull request and check validity
    // ...
}

// Function to check if the pull request is valid from the json data stored
// This takes PR number and username, then checks for matching instances
pub fn is_valid_pull_request_json(json_file_data: &[u8], pr_username: &str, pr_num: u32) -> Result<(String, String)> {
    // Convert the byte array into a string
    let json_string = std::str::from_utf8(json_file_data).unwrap();

    // Use the serde_json::from_str function to parse the JSON string into a Rust struct
    let list: List = serde_json::from_str(json_string).unwrap();

    // Iterate through the profiles in the struct and find the one that has the pull request username
    let profile = list.profiles.iter().find(|p| p.github == pr_username);
    if profile.is_none() {
        return Err(anchor_lang::error::Error::new(format!("Profile not found for username: {}", pr_username)));
    }
    let wallet_address = profile.unwrap().solanaAddress;

    // Use the Github API to check if the pull request is open and the user/assignee that opened
    // ...

    Ok((list.name, wallet_address))
}

// Function to extract the Github username from the instruction data
pub fn extract_username_instruction(instruction: &AnchorInstruction) -> String {
    let (username, pull_request_id) = match instruction.data {
        Ok(data) => {
            let mut cursor = Cursor::new(data);
            let username_len = cursor.read_u8().unwrap() as usize;
            let username = cursor.get_ref()[..username_len]
                .to_vec();
            cursor.set_position(username_len as u64);
            let pull_request_id = cursor.read_u8().unwrap();
            (username, pull_request_id)
        },
        Err(_) => return "".to_string(),
    };
    // Use the Github API to get the pull request and extract the username
    // ...
    pull_request_id
}

// Function to extract the GitHub username from the json file data
pub fn extract_username_json(json_file_data: &[u8], pr_num: u32) -> Result<(String, String)> {
    // Convert the byte array into a string
    let json_string = std::str::from_utf8(json_file_data).unwrap();

    // Use the serde_json::from_str function to parse the JSON string into a Rust struct
    let list: List = serde_json::from_str(json_string).unwrap();

    // Use the Github API to get the pull request and extract the username
    // ...
    for profile in &list.profiles {
       let pull_request = octocrab::find_pr(pr_num); // replace with actual octocrab api call
       let username = pull_request.username; // replace with actual octocrab api
    //    let profile.github == username {
        return Ok(username)
    //    }
    
    }

    return Err(anchor_lang::error::Error::new("No matching profile found for pull request"));
}

pub fn extract_wallet_json(json_file_data: &[u8], pr_num: u32) -> Result<(String, String)> {
    // Convert the byte array into a string
    let json_string = std::str::from_utf8(json_file_data).unwrap();

    // Use the serde_json::from_str function to parse the JSON string into a Rust struct
    let list: List = serde_json::from_str(json_string).unwrap();

    // Use the Github API to get the pull request and extract the username
    // ...
    for profile in &list.profiles {
        let pull_request = octocrab::find_pr(pr_num); // replace with actual octocrab api call
        let username = pull_request.username; // replace with actual octocrab api
        let wallet_address = pull_request.solana_wallet_address;
        // let profile.solana_wallet_address == wallet_address {
         return Ok(wallet_address)
        // }
     
     }
 
     return Err(anchor_lang::error::Error::new("No matching profile found for pull request"));
 
}

pub fn get_nft_metadata(instruction: &AnchorInstruction, wallet_address: &str) -> Result<String> {
    let client = AnchorClient::new();

    let username = extract_username_instruction(&instruction);
    let wallet_address = check_github_wallet(username);
    let metadata = client.get_metadata(instruction, wallet_address)?;
    Ok(metadata.to_string())
}


pub fn check_github_wallet(instruction: &AnchorInstruction) -> bool {
    // Add logic here
    true
}

pub fn check_bonk(instruction: &AnchorInstruction) -> bool {
    // Add logic here
    true
}

pub fn request_escrow(instruction: &AnchorInstruction) {
    // Add logic here
}


// Function to release the escrow to the wallet of the username
pub fn release_escrow(username: &str) {
    // Get the wallet account from the username
    // call the function to get nft metadata to release to OR call get wallet address from username 
    let result = Vec::<u8>::new();
}

pub fn check_merge_commit(username: &str, pull_request_id: u64) -> bool {
    // Add logic here to check if the merge commit was accepted and merged
    true
}

#[entrypoint]
pub fn entrypoint(_input: &[u8]) -> Vec<u8> {
    let json_data = read_json_file(_input);
    JSON_FILE_DATA = json_data.as_slice();
    let instruction = AnchorInstruction::deserialize(_input).unwrap();
    let pr_username = extract_username_instruction(&instruction);
    let is_valid = is_valid_pull_request_insruction(&instruction);
    if !is_valid {
        println!("Invalid pull request");
        return anchor_lang::error::Error::new("Invalid pull request instruction")
            .to_output_vec();
    }
    if !check_github_wallet(&instruction) {
        println!("Github and wallet link do not match");
        return anchor_lang::error::Error::new("Invalid github wallet link")
            .to_output_vec();
    }
    if !check_bonk(&instruction) {
        println!("Pull request does not contain the word 'bonk'");
        return anchor_lang::error::Error::new("Invalid bonk reward in pull request")
            .to_output_vec();
    }
    request_escrow(&instruction);
    if check_merge_commit(&pr_username, instruction.pull_request_id) {
        release_escrow(&pr_username);
    }
    return JSON_FILE_DATA
}

fn main() {
    let input = vec![]; //create a function that handles input from relayer fed by monitor
    entrypoint(&input);
}

