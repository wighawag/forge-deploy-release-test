use std::{fs, path::Path};

use path_slash::PathExt;
use regex::Regex;
use substring::Substring;
use walkdir::WalkDir;

use crate::types::{ConstructorObject, ContractObject, InputObject};

struct ContractName {
    start: usize,
    end: usize,
    name: String,
}

pub fn get_contracts(root_folder: &str, sources_folder: &str) -> Vec<ContractObject> {
    let match_comments = Regex::new(r#"(?m)(".*?"|'.*?')|(/\*.*?\*/|//[^\r\n]*$)"#).unwrap(); //gm
    let match_strings = Regex::new(r#"(".*?"|'.*?')"#).unwrap(); //g
    let match_contract_names = Regex::new(r#"(?m)contract[\s\r\n]*(\w*)[\s\r\n]"#).unwrap(); // gm
    let per_contract_match_constructor =
        Regex::new(r#"(?s)constructor[\s\r\n]*\((.*?)\)"#).unwrap(); // gs

    let folder_path_buf = Path::new(root_folder).join(sources_folder);
    let folder_path = folder_path_buf.to_str().unwrap();

    // println!("generating deployer from {folder_path} ...");

    let mut contracts: Vec<ContractObject> = Vec::new();

    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_file() && entry.path().extension().unwrap().eq("sol") {
            let data = fs::read_to_string(entry.path()).expect("Unable to read file");
            let data = match_comments.replace_all(&data, "");
            let data = match_strings.replace_all(&data, "");

            let mut contract_name_objects: Vec<ContractName> = Vec::new();

            let mut i = 0;
            for contract_names in match_contract_names.captures_iter(&data) {
                if let Some(the_match) = contract_names.get(0) {
                    if let Some(first_group) = contract_names.get(1) {
                        let contract_name = first_group.as_str();
                        let start = the_match.start();
                        if i > 0 {
                            contract_name_objects[i - 1].end = start;
                        }
                        contract_name_objects.push(ContractName {
                            name: String::from(contract_name),
                            start: start,
                            end: data.len(),
                        });
                        i = i + 1;
                    }
                }
            }

            for contract_name_object in contract_name_objects {
                let contract_string =
                    data.substring(contract_name_object.start, contract_name_object.end);

                let constructor_string =
                    match per_contract_match_constructor.captures(contract_string) {
                        Some(found) => match found.get(1) {
                            Some(constructor) => {
                                let result = constructor.as_str().trim();
                                if result.eq("") {
                                    None
                                } else {
                                    Some(result.to_string())
                                }
                            }
                            None => None,
                        },
                        None => None,
                    };

                let parsable_constructor_string =
                    constructor_string.clone().unwrap_or("".to_string());
                let args = parsable_constructor_string
                    .split(",")
                    .map(|s| s.trim().split(" ").last().unwrap());

                // println!("constructor_string: '{}'", constructor_string.clone().unwrap_or("NO".to_string()));

                let solidity_filepath = entry.path().to_slash().unwrap().to_string();
                let solidity_filepath = solidity_filepath.substring(2, solidity_filepath.len());
                let contract = ContractObject {
                    solidity_filepath: String::from(solidity_filepath),
                    contract_name: String::from(contract_name_object.name),
                    solidity_filename: String::from(entry.file_name().to_str().unwrap()),
                    constructor: Some(ConstructorObject {
                        inputs: args
                            .map(|arg| InputObject {
                                name: String::from(arg),
                                r#type: None,
                            })
                            .collect(),
                    }),
                    constructor_string: constructor_string,
                };
                // println!("{:?}", contract);
                contracts.push(contract);
            }
        }
    }
    return contracts;
}
