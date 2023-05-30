// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.3.0 <0.9.0;

import "../node_modules/@openzeppelin/contracts/access/AccessControl.sol";
import "../node_modules/@openzeppelin/contracts/access/Ownable.sol";

// import "@openzeppelin/contracts/access/Roles.sol";

contract Policy is AccessControl {

    bytes32 public constant VIDEO_WORKSPACE_ADMIN = keccak256("VIDEO_WORKSPACE_ADMIN");
    bytes32 public constant AUDIO_WORKSPACE_ADMIN = keccak256("AUDIO_WORKSPACE_ADMIN");

    bytes32 public constant SEND_VIDEO = keccak256("VIDEO_SENDER_ROLE");
    bytes32 public constant RECEIVE_VIDEO = keccak256("VIDEO_RECEIVER_ROLE");
    bytes32 public constant SEND_AUDIO = keccak256("AUDIO_SENDER_ROLE");
    bytes32 public constant RECEIVE_AUDIO = keccak256("AUDIO_RECEIVER_ROLE");
    bytes32 public constant SEND_TEXT = keccak256("TEXT_SENDER_ROLE");
    bytes32 public constant RECEIVE_TEXT = keccak256("TEXT_RECEIVER_ROLE");

    address video_admin;
    address audio_admin;

    int16[] tokens = new int16[](0);

    int16 counter = 0;

    event MyEvent (
        address video_admin_event,
        address audio_admin_event,
        int16[] tokens
    );
    
    event AuthTokens (
        bytes32 tokens
    );

    constructor(address default_video_admin, address default_audio_admin) {

        video_admin = default_video_admin;
        audio_admin = default_audio_admin;

        _grantRole(VIDEO_WORKSPACE_ADMIN, video_admin);
        _grantRole(AUDIO_WORKSPACE_ADMIN, audio_admin);

    }

    // function push_new_token() external{
    //     // counter++;
    //     // tokens.push(counter);
    // }

    function get_default_admins() external {
        // counter++;
        // tokens.push(counter);
        emit MyEvent(video_admin, audio_admin, tokens);
        // emit MyEvent(video_admin);
    }

    function check_send_video(address addr) public view returns(bytes32) {
        require(hasRole(SEND_VIDEO, addr), "Access Denied");
        return "Access Granted";
    }

    function check_receive_video(address addr) public view returns(bytes32) {
        require(hasRole(RECEIVE_VIDEO, addr), "Access Denied");
        return "Access Granted";
    }

    function check_send_audio(address addr) public view returns(bytes32) {
        require(hasRole(SEND_AUDIO, addr), "Access Denied");
        return "Access Granted";
    }

    function check_receive_audio(address addr) public view returns(bytes32) {
        require(hasRole(RECEIVE_AUDIO, addr), "Access Denied");
        return "Access Granted";
    }

    function grantRole(bytes32 role, address account) public override  {
        _grantRole(role, account);
    }

    function Grant_Video_Sender_Role(address receiver) public {
        require(hasRole(VIDEO_WORKSPACE_ADMIN, msg.sender));
        grantRole(SEND_VIDEO, receiver);
    }

    function Grant_Video_Receiver_Role(address receiver) public {
        require(hasRole(VIDEO_WORKSPACE_ADMIN, msg.sender));
        grantRole(RECEIVE_VIDEO, receiver);
    }

    function Grant_Audio_Sender_Role(address receiver) public {
        require(hasRole(AUDIO_WORKSPACE_ADMIN, msg.sender));
        grantRole(SEND_AUDIO, receiver);
    }

    function Grant_Audio_Receiver_Role(address receiver) public {
        require(hasRole(AUDIO_WORKSPACE_ADMIN, msg.sender));
        grantRole(RECEIVE_AUDIO, receiver);
    }

    function Clear_Tokens() public {
        require(hasRole(AUDIO_WORKSPACE_ADMIN, msg.sender));
        require(hasRole(VIDEO_WORKSPACE_ADMIN, msg.sender));
        delete tokens;
    }

    function Generate_Token(int16 random_num) public {
        require(hasRole(AUDIO_WORKSPACE_ADMIN, msg.sender));
        require(hasRole(VIDEO_WORKSPACE_ADMIN, msg.sender));
        tokens.push(random_num);
    }

    function Check_Token(int16 token_val) public view returns(bytes32) {
        for (uint i=0; i<tokens.length; i++) {
            if (tokens[i] == token_val) {
                return "Valid";
            }
        }
        return "Not Valid";
    }

    function get_number() public view returns(uint) {
        return 0;
    }

}