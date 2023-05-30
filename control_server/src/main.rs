use std::{fs, any};
use std::future::Future;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::ops::Deref;
use web3::Transport;
use web3::contract::tokens::Detokenize;
use web3::transports::Http;
use web3::{
    contract::{Contract, Options},
    types::U256,
};
use web3::types::{Address, Bytes};

use serde_json::{Result, Value};

use std::borrow::Borrow;

use rand::{thread_rng, Rng};


async fn get_accounts() -> web3::Result<Vec<web3::types::H160>> {
    let transport =  web3::transports::Http::new("http://127.0.0.1:7545")?;
    let web3 = web3::Web3::new(transport);

    println!("Calling accounts.");
    let accounts: Vec<web3::types::H160> = match web3.eth().accounts().await {
        Ok(res) => res,
        Err(e) => panic!("{:?}",e),
    };

    
    Ok(accounts)

}


async fn connect_to_contract() -> web3::contract::Result<web3::contract::Contract<Http>> {
    let transport =  web3::transports::Http::new("http://127.0.0.1:7545")?;
    let web3 = web3::Web3::new(transport);

    println!("Calling accounts.");
    let accounts = web3.eth().accounts().await?;
    // println!("Accounts: {:?}", accounts);

    // for account in accounts {
    //     let balance = web3.eth().balance(account, None).await?;
    //     println!("Balance of {:?}: {}", account, balance);
    // }

    let file = fs::File::open("Policy.json")
        .expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");
    let contract_abi: &Value = &json["abi"];

    let id = match web3.eth().chain_id().await {
        Ok(id) => id,
        Err(error) => panic!("Problem retrieving network id, {:?}", error),
    };


    let address: &Value = &json["networks"][id.as_usize()]["address"];

    println!("network address = {:?}", address);

    let abi_str = contract_abi.to_string();


    let decoded_abi =  abi_str.as_bytes();

    println!("decoded_abi = {:?}", abi_str);
    
    let mut buffer = [0; 20];
    match address.to_string().as_bytes().read(&mut buffer) {
        Ok(v) => v,
        Err(error) => panic!("{:?}", error),
    };

    let network_address: web3::types::Address = "0x63f8a8145DCa03aED46439a5e46d326961aBda3e".parse().expect("Invalid address");

    // println!("network address = {:?}", id.as_usize());

    let contract_enum = web3::ethabi::Contract::load(decoded_abi);

    let contract_enum_contract = match contract_enum {
        Ok(v) => v,
        Err(e) => panic!("{:?}",e),
    };

    let contract_ = web3::contract::Contract::new(web3.eth(), network_address, contract_enum_contract);
    
    let address_string = "0xdb389bc8fb32742e8bd3444299c2b538c4f3ea2f";
    let decoded_address: web3::types::Address = address_string.parse().expect("Invalid address");
    

    // for account in accounts {
        // contract_.query("get_number", (), None, Options::default(), None).await?;
        // println!("contract_ res {:?}", res);
    // }

    Ok(contract_)
}

#[tokio::main]
async fn main() -> web3::contract::Result<()>{

    

    // listens with TCP
    let connection_listener = TcpListener::bind("127.0.0.1:20012").unwrap();

    // init smart contract
    let contract: Contract<Http> = match connect_to_contract().await {
        Ok(result) => result,
        Err(error) => panic!("Problem connecting to blockchain network, {:?}", error),
    };

    let accounts = match get_accounts().await {
        Ok(result) => result,
        Err(error) => panic!("Problem retrieving accounts, {:?}", error),
    };

    println!("contract = {:?}", contract);

    let mut address_buffer = [0; 20];
    let mut address_str = "0xdb389bc8fb32742e8bd3444299c2b538c4f3ea2f".as_bytes();
    address_str.read(&mut address_buffer);
    // let decoded_address: web3::types::H160 = web3::types::H160(address_buffer);

    let transport =  web3::transports::Http::new("http://127.0.0.1:7545");


    let address_string = "0xdb389bc8fb32742e8bd3444299c2b538c4f3ea2f";
    let decoded_address: web3::types::Address = address_string.parse().expect("Invalid address");

    println!("Address: {:?}", decoded_address);

    // let res = contract.call("get_default_admins", (), decoded_address, Options::default()).await;

    // Test random number
    // let mut rng = thread_rng();
    // let x: u32 = rng.gen_range(100..=9999);
    // println!("random = {}", x);
    // ------------------

    // Listening for request
    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection Established");
        let mut buffer = [0; 200];
        stream.read(&mut buffer).unwrap();
        print!("Received from client: {}", String::from_utf8_lossy(&buffer));

        let request_ = buffer.to_vec();
        
        
        let mut data: Vec<u8> = Vec::new();

        let mut i = 0;
        let mut start: bool = false;
        let mut index = 0;

        while start == false {
            if request_[i] == 123 {
                index = i;
                start = true;
            }
            i+=1;
        }

        i = 0;

        println!("index = {:?}", index);

        while i < 50 && start == true {
            data.push(request_[index]);
            index = index + 1;
            if request_[index] == 125 {
                println!("end here");
                data.push(request_[index]);
                start = false;
            }
            
        }

        let request = String::from_utf8(data).unwrap();

        println!("request = {}", request);

        println!();
        
        let request_str: &str = request.as_str();


        println!("request_str = {:?}", request_str);

        let v: Value =  match serde_json::from_str(request_str) {
            Ok(res) => res,
            Err(e) => panic!("Error = {:?}", e),
        };

        if v.get("request").is_none() == false {
            println!("request = {:?}", v["request"]);
        }

        if v.get("address").is_none() == false {
            println!("address = {:?}", v["address"]);
        }

        // requests = {create_audio_sender, create_audio_receiver, create_video_sender, create_video_receiver}

        if v["request"] == "create_audio_sender" {
            // contract -> check_send_audio()
            println!("request = create_audio_sender");
            // let mut address_buffer = [0; 20];
            // let mut sender_address = v["address"];

            let receiver_address: web3::types::Address = v["address"].as_str().unwrap().to_string().parse().expect("Invalid address");

            println!("receiver_address = {:?}", receiver_address);

            let temp: web3::types::H256 = web3::types::H256::zero();

            let res = match contract.call("check_send_audio", (receiver_address), decoded_address, Options::default()).await {
                Ok(res) => res,
                Err(e) => temp
            };

            if (res == temp) {
                let mut response_buffer = [0; 200];
                let mut response = "Not allowed ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            } else {

                // Generate token || stream_id
                let mut rng = thread_rng();
                let token: i16 = rng.gen_range(100..=9999);
                println!("Generate token = {:?}", token);
                contract.call("Generate_Token", (token), decoded_address, Options::default()).await.unwrap();

                let mut response_buffer = [0; 200];
                let mut response = "Success ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            }

            println!("decoded_address = {:?}", decoded_address);

        //     let gas_limit: U256 = 1000000.into();
        //     let options = Options::with(|opt| {
        //         opt.gas = Some(gas_limit);
        //     });

        //     let result = contract.query("push_new_token", (), None, options, None);
        //     let states: web3::types::Bytes = match result.await {
        //         Ok(res) =>  res,
        //         Err(e) => panic!("Error here = {:?}", e),
        //     };

            println!("create_audio_sender, res = {:?}", res);
        } 
        else if v["request"] == "create_audio_receiver" {
            // contract -> check_receive_audio()
            println!("request = create_audio_receiver");

            let receiver_address: web3::types::Address = v["address"].as_str().unwrap().to_string().parse().expect("Invalid address");

            println!("receiver_address = {:?}", receiver_address);

            let temp: web3::types::H256 = web3::types::H256::zero();

            let res = match contract.call("check_receive_audio", (receiver_address), decoded_address, Options::default()).await {
                Ok(res) => res,
                Err(e) => temp
            };

            if (res == temp) {
                let mut response_buffer = [0; 200];
                let mut response = "Not allowed ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            } else {
                // Generate token || stream_id
                let mut rng = thread_rng();
                let token: i16 = rng.gen_range(100..=9999);
                println!("Generate token = {:?}", token);
                contract.call("Generate_Token", (token), decoded_address, Options::default()).await.unwrap();

                let mut response_buffer = [0; 200];
                let mut response = "Success ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            }

        }
        else if v["request"] == "create_video_sender" {
            // check_send_video()
            println!("request = create_video_sender");
        
            let receiver_address: web3::types::Address = v["address"].as_str().unwrap().to_string().parse().expect("Invalid address");

            println!("receiver_address = {:?}", receiver_address);

            let temp: web3::types::H256 = web3::types::H256::zero();

            let res = match contract.call("check_send_video", (receiver_address), decoded_address, Options::default()).await {
                Ok(res) => res,
                Err(e) => temp
            };

            if (res == temp) {
                let mut response_buffer = [0; 200];
                let mut response = "Not allowed ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            } else {
                // Generate token || stream_id
                let mut rng = thread_rng();
                let token: i16 = rng.gen_range(100..=9999);
                println!("Generate token = {:?}", token);
                contract.call("Generate_Token", (token), decoded_address, Options::default()).await.unwrap();

                let mut response_buffer = [0; 200];
                let mut response = "Success ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            }

        }
        else if v["request"] == "create_video_receiver" {
            // contract -> check_receive_video()
            print!("request = create_video_receiver");

            let receiver_address: web3::types::Address = v["address"].as_str().unwrap().to_string().parse().expect("Invalid address");

            println!("receiver_address = {:?}", receiver_address);

            let temp: web3::types::H256 = web3::types::H256::zero();

            let res = match contract.call("check_receive_video", (receiver_address), decoded_address, Options::default()).await {
                Ok(res) => res,
                Err(e) => temp
            };

            if (res == temp) {
                let mut response_buffer = [0; 200];
                let mut response = "Not allowed ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            } else {
                // Generate token || stream_id
                let mut rng = thread_rng();
                let token: i16 = rng.gen_range(100..=9999);
                println!("Generate token = {:?}", token);
                contract.call("Generate_Token", (token), decoded_address, Options::default()).await.unwrap();
                
                let mut response_buffer = [0; 200];
                let mut response = "Success ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            }

            println!("decoded_address = {:?}", decoded_address);
        }
        else if v["request"] == "Grant_Video_Sender_Role" {
            print!("request = Grant_Video_Sender_Role");

            let receiver_address: web3::types::Address = v["address"].as_str().unwrap().to_string().parse().expect("Invalid address");

            println!("receiver_address = {:?}", receiver_address);

            let temp: web3::types::H256 = web3::types::H256::zero();

            let res = match contract.call("Grant_Video_Sender_Role", (receiver_address), decoded_address, Options::default()).await {
                Ok(res) => res,
                Err(e) => temp
            };

            if (res == temp) {
                let mut response_buffer = [0; 200];
                let mut response = "Not allowed ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            } else {
                let mut response_buffer = [0; 200];
                let mut response = "Success ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            }

        }
        else if v["request"] == "Grant_Video_Receiver_Role" {
            print!("request = Grant_Video_Receiver_Role");

            let receiver_address: web3::types::Address = v["address"].as_str().unwrap().to_string().parse().expect("Invalid address");

            println!("receiver_address = {:?}", receiver_address);

            let temp: web3::types::H256 = web3::types::H256::zero();

            let res = match contract.call("Grant_Video_Receiver_Role", (receiver_address), decoded_address, Options::default()).await {
                Ok(res) => res,
                Err(e) => temp
            };

            if (res == temp) {
                let mut response_buffer = [0; 200];
                let mut response = "Not allowed ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            } else {
                let mut response_buffer = [0; 200];
                let mut response = "Success ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            }
        }
        else if v["request"] == "Grant_Audio_Sender_Role" {
            print!("request = Grant_Audio_Sender_Role");

            let receiver_address: web3::types::Address = v["address"].as_str().unwrap().to_string().parse().expect("Invalid address");

            println!("receiver_address = {:?}", receiver_address);

            let temp: web3::types::H256 = web3::types::H256::zero();

            let res = match contract.call("Grant_Audio_Sender_Role", (receiver_address), decoded_address, Options::default()).await {
                Ok(res) => res,
                Err(e) => temp
            };

            if (res == temp) {
                let mut response_buffer = [0; 200];
                let mut response = "Not allowed ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            } else {
                let mut response_buffer = [0; 200];
                let mut response = "Success ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            }
        }
        else if v["request"] == "Grant_Audio_Receiver_Role" {
            print!("request = Grant_Audio_Receiver_Role");

            let receiver_address: web3::types::Address = v["address"].as_str().unwrap().to_string().parse().expect("Invalid address");

            println!("receiver_address = {:?}", receiver_address);

            let temp: web3::types::H256 = web3::types::H256::zero();

            let res = match contract.call("Grant_Audio_Receiver_Role", (receiver_address), decoded_address, Options::default()).await {
                Ok(res) => res,
                Err(e) => temp
            };

            if (res == temp) {
                let mut response_buffer = [0; 200];
                let mut response = "Not allowed ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            } else {
                let mut response_buffer = [0; 200];
                let mut response = "Success ______________".as_bytes();
                response.read(&mut response_buffer).unwrap();
                stream.write(&mut response_buffer).unwrap();
            }
        }
        else if v["request"] == "Clear_Tokens" {
            let temp: web3::types::H256 = web3::types::H256::zero();

            let res = match contract.call("Clear_Tokens", (), decoded_address, Options::default()).await {
                Ok(res) => res,
                Err(e) => temp
            };
        }

        // stream.write(&mut buffer).unwrap();
    }


    Ok(())
}

// Requests Template
// "{"request":"Clear_Tokens"}"
// "{"request":"Grant_Audio_Sender_Role","address":"0x59d933327E78eF41D86181CA6FEc35562264bBE5"}"
// "{"request":"create_audio_sender", "address":"0x59d933327E78eF41D86181CA6FEc35562264bBE5"}"
// "{"request":"create_audio_sender", "address":"0x97Dd8feC6e52065700797D00e372eD318AbAd1AB"}"