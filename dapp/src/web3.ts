import { deploy } from './deploy-contract-web3';
import { getContractInfo, callContract, initWeb3, deployContract } from './call-contract-web3';
// deploy contract
(async function () {
    let { web3, account } = await initWeb3("wss://ethereum-sepolia-rpc.publicnode.com");
    // let { contract, abi } = await deployContract(web3, account);
    let contract = "0x329f6ec542a232c44572659a8c47bd04beb02bc7";
    let { abi } = getContractInfo();
    if (contract)
        await callContract(web3, account, abi, contract);
})()
