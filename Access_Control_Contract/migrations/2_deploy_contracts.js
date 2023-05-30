var Policy = artifacts.require("Policy");

module.exports = function(deployer) {
    deployer.deploy(Policy, 
        "0xDb389bc8FB32742E8BD3444299C2b538C4F3eA2f",
        "0xDb389bc8FB32742E8BD3444299C2b538C4F3eA2f"
    );
}