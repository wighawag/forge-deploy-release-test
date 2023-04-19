// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;
import {Vm} from "forge-std/Vm.sol";
import "forge-std/console.sol";



struct DeployerDeployment {
    string name;
    address addr;
    string artifactPath;
    string artifactContractName;
}

struct DeployOptions {
    bool overrideIfExist;
}

contract Deployer {
    Vm constant vm =
        Vm(address(bytes20(uint160(uint256(keccak256("hevm cheat code"))))));

    mapping(string => DeployerDeployment) _namedDeployments;
    DeployerDeployment[] _newDeployments;

    string internal deploymentContext;
    function newDeployments() external view returns (DeployerDeployment[] memory) {
        return _newDeployments;
    }

    function _getDeploymentContext() private returns (string memory context) {
        // no deploymentContext provided we fallback on chainID
        uint256 currentChainID;
        assembly {
            currentChainID := chainid()
        }
        context = vm.envOr("DEPLOYMENT_CONTEXT", vm.toString(currentChainID));
    } 


    constructor() {
        deploymentContext = _getDeploymentContext();
    }

    // external but not supposed to be called 
    function _preCheck(string memory name, DeployOptions memory options) external {
        if (!options.overrideIfExist) {
            DeployerDeployment memory existing = _namedDeployments[name];
            if (existing.addr != address(0)) {
                // TODO option to override, // ask for input ?
                revert("Using same deployment name in the same script");
            }
        }

        vm.broadcast();
    }


    function hasDeployed(
        string memory name
    ) public returns (bool) {
        // TODO use Deployment json files ?
        // or generated Deployments.g.sol
        return false;
    }

    // --------------------------------------------------------------------------------------------
    // also expose the save function that can save deployment info to disk
    // --------------------------------------------------------------------------------------------
    function save(
        string memory name,
        address deployed,
        string memory artifactPath,
        string memory artifatContractName
    ) public {
        DeployerDeployment memory deployment = DeployerDeployment({
            name: name,
            addr: address(deployed),
            artifactPath: artifactPath,
            artifactContractName: artifatContractName
        });
        _namedDeployments[name] = deployment;
        _newDeployments.push(deployment);
        save(deployment);
    }

    function save(
        string memory name,
        address deployed
    ) public {
        DeployerDeployment memory deployment = DeployerDeployment({
            name: name,
            addr: address(deployed),
            artifactPath: "",
            artifactContractName: ""
        });
        _namedDeployments[name] = deployment;
        _newDeployments.push(deployment);
        save(deployment);
    }

    function save(DeployerDeployment memory deployment) internal {
        string memory artifactAsString = vm.readFile(
            string(
                bytes.concat(
                    "out/",
                    bytes(deployment.artifactPath),
                    "/",
                    bytes(deployment.artifactContractName),
                    ".json"
                )
            )
        );
        bytes memory artifact = vm.parseJson(artifactAsString);
        string memory artifactABI = abi.decode(artifact, (string));
        // bytes memory artifactABI = vm.parseJson(artifactAsString, "abi");
        string memory json = deployment.name;
        vm.serializeAddress(json, "address", deployment.addr);
        // string memory finalJson = vm.serialize(json, "abi", artifactABI);
        // vm.writeFile(
        //     string(bytes.concat("./deployments/", bytes(name), ".json")),
        //     string(artifactABI)
        // );
        console.log(string(artifactABI));
    }
    // --------------------------------------------------------------------------------------------


    // --------------------------------------------------------------------------------------------
    // --------------------------------------------------------------------------------------------
    // GENERATED
    // --------------------------------------------------------------------------------------------
    // --------------------------------------------------------------------------------------------
    

    
    // --------------------------------------------------------------------------------------------
    // --------------------------------------------------------------------------------------------
}