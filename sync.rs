use std::collections::HashMap;
use std::fs;
use std::{path::Path};

use crate::types::{DeploymentObject, ArtifactJSON, DeploymentJSON};

pub fn generate_deployments(root_folder: &str, deployment_folder: &str, artifacts_folder: &str, new_deployments: &HashMap<String, DeploymentObject>) {
    let out_folder_path_buf = Path::new(root_folder).join(deployment_folder);
    let artifact_folder_path_buf = Path::new(root_folder).join(artifacts_folder);

    for (key, value) in new_deployments.iter() {
        let folder_path_buf = out_folder_path_buf.join(value.deployment_context.as_str());
        fs::create_dir_all(&folder_path_buf).expect("could not create folder");
        let chainid_file_path_buf = folder_path_buf.join(".chainId");
        if !chainid_file_path_buf.exists() {
            fs::write(chainid_file_path_buf, &value.chain_id).expect("failed to write the .chainId file");
        }

        // unfortunately forge do not export artifacts in the broadcast file, so we have to fetch in the out folder
        // if sync is called not directly, out folder could be out of sync and we would get wrong artifact data
        // TODO save artifact in the solidity execution in temporary files and fetch artifact data from there 
        let artifact_path_buf = artifact_folder_path_buf.join(&value.artifact_path).join(format!("{}.json", value.contract_name));
        let data = fs::read_to_string(artifact_path_buf).expect("Unable to read file");
        let artifact: ArtifactJSON = serde_json::from_str(&data).expect("Unable to parse");


        let file_path_buf = folder_path_buf.join(format!("{}.json", key));

        let data = serde_json::to_string_pretty(&DeploymentJSON {address: value.address.to_string(), abi: artifact.abi}).expect("Failed to stringify");
        fs::write(file_path_buf, data).expect("failed to write file");
    }
}