import logo from './logo.svg';
import './App.css';

import Web3 from 'web3';
import { useEffect, useState } from 'react';
const MyContract = require('./Policy.json');


function App() {

  const [default_video_admin, set_default_video_admin] = useState("");
  const [default_audio_admin, set_default_audio_admin] = useState("");
  const [latest_request_access, set_latest_request_access] = useState("");
  const [existing_streams, set_existing_streams] = useState([]);
  const [network_address, set_network_address] = useState("");
  const [retrieve, set_retrieve] = useState(0);

  const init = async () => {

    console.log("init()");

    const web3 = new Web3("http://127.0.0.1:7545");
    
    const id = await web3.eth.net.getId();

    console.log("id = ", id);
    
    const deployedNetwork = MyContract.networks[id];
    const contract = new web3.eth.Contract(
      MyContract.abi,
      deployedNetwork.address
    );

    console.log("deployedNetwork.address = ", deployedNetwork.address);

    const addresses = await web3.eth.getAccounts();
    await contract.methods.get_default_admins().send({ from: addresses[0], gas: '1000000' });
    

    contract.events.MyEvent({})
      .on("data", event => console.log(event));

    const results = await contract.getPastEvents(
      'MyEvent',
      {fromBlock: 0}
    );

    console.log("results = ", results[results.length-1]);
    console.log("results.length = ", results.length);

    console.log("default_video_admin = ", results[results.length-1].returnValues.video_admin_event);
    console.log("default_audio_admin = ", results[results.length-1].returnValues.audio_admin_event);
    console.log("tokens = ", results[results.length-1].returnValues.tokens);

    set_default_video_admin(results[results.length-1].returnValues.video_admin_event);
    set_default_audio_admin(results[results.length-1].returnValues.audio_admin_event);
    set_latest_request_access(results[results.length-1].address);
    set_network_address(deployedNetwork.address);

    


    // await contract.methods.get_default_admins().send({ from: addresses[0] });

    // await contract.methods.push_new_token().send({ from: addresses[1] });

    contract.events.AuthTokens({})
      .on("data", event => console.log(event));

    const token_results = await contract.getPastEvents(
      'MyEvent',
      {fromBlock: 0}
    );

    console.log("tokens array = ", token_results[token_results.length-1].returnValues.tokens);
    for (let i=0; i<token_results[token_results.length-1].returnValues.tokens.length; i++){
      set_existing_streams(existing_streams.push(token_results[token_results.length-1].returnValues.tokens[i]))
    }
    set_existing_streams(token_results[token_results.length-1].returnValues.tokens);
    console.log("existing_streams = ", existing_streams);

    // await contract.methods.Grant_Audio_Sender_Role(addresses[1]).send({ from: addresses[0], gas: '1000000' }).catch( (e) => {console.log(e)});

    // let res = await contract.methods.check_send_audio(addresses[2]).send({ from: addresses[0], gas: '1000000' }).catch( (e) => {console.log(e)})
    
    // console.log("res method = ", res);

    await new Promise(resolve => setTimeout(resolve, 2000));

  }

  useEffect(() => {
    if (retrieve <= 2)
    {
      init();
      set_retrieve(retrieve+1);
    }
  });
  

  
  return (
    <div>
      <h1>Public Access Control Policy</h1>
      <table>
         
        <tr>
          <th><h2>State</h2></th>
          <th><h2>Value</h2></th>
        </tr>

        <tr>
          <th>Deployed Network Address</th>
          <th>{network_address}</th>
        </tr>

        <tr>
          <th>Default Video Admin</th>
          <th>{default_video_admin}</th>
        </tr>

        <tr>
          <th>Default Audio Admin</th>
          <th>{default_audio_admin}</th>
        </tr>

        <tr>
          <th>Latest access request </th>
          <th>{latest_request_access}</th>
        </tr>

        <tr>
          <th>Existing streams</th>
          <th>
            {existing_streams.map(item => {
              return <p>{item}</p>
            })}
          </th>
        </tr>

      </table>
    </div>
  );
}

export default App;
