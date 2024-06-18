import Web3 from 'web3';
import * as fs from "fs";
import dotnev from "dotenv";
import { normalize } from 'path';
const solc = require('solc');

// init env
const rpc = "wss://ethereum-sepolia-rpc.publicnode.com";

function getPrivateKey(): string {
    dotnev.config();
    let privateKey = process.env.priv_key;
    if (privateKey && privateKey.slice(0, 2) !== '0x')
        privateKey = '0x' + privateKey;
    else
        privateKey = ""
    console.log(privateKey);
    return privateKey;
}


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

// 借助web3进行链上的交互操作
const web3 = new Web3(rpc);
const accounts = web3.eth.accounts.wallet.add(getPrivateKey());

// 发布合约
export const deploy = async () => {
    const contract = new web3.eth.Contract(abi);
    const deployTx = contract.deploy({
        data: '0x' + bytecode,
        arguments: [0], // init contract args
    });
    const gas = await deployTx.estimateGas({
        from: accounts[0].address
    });
    const value = await web3.eth.getBalance(accounts[0].address);
    console.log(`estimate gas: ${gas} address: ${accounts[0].address}, native value: ${value}`);
    try {
        const tx = await deployTx.send({
            from: accounts[0].address,
        })
        console.log(`contract deployed: ${JSON.stringify(tx.options)}`);
    } catch (err) {
        console.error(err);
    }
}
