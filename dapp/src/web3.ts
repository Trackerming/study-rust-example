import { deploy } from './deploy-contract-web3';
import { getContractInfo, callContract, initWeb3, deployContract } from './call-contract-web3';
import { getWallet, callContract as callContractByEthers } from './call-contract-ethers';
// deploy contract
(async function () {
    const rpc = "https://rpc2.sepolia.org";
    let { web3, account } = await initWeb3(rpc);
    // let { contract, abi } = await deployContract(web3, account);
    let contract = "0x329f6ec542a232c44572659a8c47bd04beb02bc7";
    let { abi } = getContractInfo();
    if (contract)
        //await callContract(web3, account, abi, contract);
        await callContractByEthers(getWallet(rpc), abi, contract);
})()
