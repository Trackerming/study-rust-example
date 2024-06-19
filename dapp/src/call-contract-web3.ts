import Web3 from "web3";
import { Web3Account } from 'web3-eth-accounts';
import { normalize } from 'path';
const solc = require('solc');
import * as fs from 'node:fs';
import dotenv from 'dotenv';

import { deploy, getPrivateKey } from './deploy-contract-web3';

export function initWeb3(rpc: string): { web3: Web3, account: Web3Account } {
    const web3 = new Web3(rpc);
    const account = web3.eth.accounts.privateKeyToAccount(getPrivateKey());
    return { web3, account };
}

export function getContractInfo(): { bytecode: any, abi: any } {
    // 读取合约
    const source = fs.readFileSync(normalize(__dirname + '/../res/Inc.sol'), 'utf-8');
    // 编译合约
    const compileOpts = {
        language: 'Solidity',
        sources: {
            'Inc.sol': {
                content: source,
            },
        },
        settings: {
            outputSelection: {
                '*': {
                    '*': ['*'],
                },
            },
        },
    };
    const comiledCode = JSON.parse(solc.compile(JSON.stringify(compileOpts)));
    const contractFile = comiledCode.contracts['Inc.sol']['Incrementer'];
    // 获取bin和abi
    const bytecode = contractFile.evm.bytecode.object;
    const abi = contractFile.abi;
    return { bytecode, abi };
}

export async function deployContract(web3: Web3, account: Web3Account): Promise<{ contract: string | undefined, abi: any }> {
    console.log(`attemp deploy contract: from ${account.address}`);
    let { bytecode, abi } = getContractInfo();
    const deployContract = new web3.eth.Contract(abi);
    const deployTx = deployContract.deploy({
        data: bytecode,
        arguments: [2],
    });
    const createTx = await web3.eth.accounts.signTransaction({
        data: deployTx.encodeABI(),
        gas: 7000000,
        gasPrice: web3.utils.toWei('10', 'gwei'),
        nonce: await web3.eth.getTransactionCount(account.address),
    }, account.privateKey);
    const createReceipt = await web3.eth.sendSignedTransaction(createTx.rawTransaction);
    console.log(`Contract deployed at address: ${createReceipt.contractAddress}, txId: ${createReceipt.transactionHash}`);
    return { contract: createReceipt.contractAddress, abi: abi }
}

export async function callContract(web3: Web3, account: Web3Account, abi: any, contractAddress: string) {
    let contractInstance = new web3.eth.Contract(abi, contractAddress);
    // listen
    let incEvent = contractInstance.events.Increment();
    incEvent.once('data', (d) => {
        console.log(`onetime event listener... ${d}`);
    });
    let subs = await web3.eth.subscribe('logs', { address: contractAddress, topics: [] });
    subs.on('data', (d) => {
        console.log(`subs logs data: ${d}`);
    })
    let num = await contractInstance.methods.getNumber().call();
    console.log(`contract current number stored is: ${num}`);
    let addVal = 8;
    let incMethod = contractInstance.methods.increment(addVal);
    let incTx = await web3.eth.accounts.signTransaction({
        to: contractAddress,
        data: incMethod.encodeABI(),
        gasPrice: await web3.eth.getGasPrice(),
        gas: 8000000,
        nonce: await web3.eth.getTransactionCount(account.address),
    }, account.privateKey);
    const incReceipt = await web3.eth.sendSignedTransaction(incTx.rawTransaction);
    console.log(`tx succ with hash: ${incReceipt.transactionHash}`);
    num = await contractInstance.methods.getNumber().call();
    console.log(`contract current number after update(inc ${addVal}) stored is: ${num}`);
}

