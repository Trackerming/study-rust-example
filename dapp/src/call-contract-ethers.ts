import { ethers } from 'ethers';
import { getPrivateKey } from './deploy-contract-web3';
import { getContractInfo } from './call-contract-web3';

export function getWallet(rpc: string): ethers.Wallet {
    const provider = new ethers.JsonRpcProvider(rpc);
    // const signer = provider.getSigner();
    const wallet = new ethers.Wallet(getPrivateKey(), provider);
    return wallet;
}

export async function callContract(wallet: ethers.Wallet, abi: any, contractAddress: string) {
    const contractInstance = new ethers.Contract(contractAddress, abi, wallet);
    const contractReader = new ethers.Contract(contractAddress, abi, wallet.provider);
    contractReader.on('Increment', (value) => {
        console.log(`listen in Increment, value ${value}`);
    })
    let num = await contractReader.getNumber();
    console.log(`ethers call contract before update num:${num}`)
    const addVal = 3;
    const incReceipt = await contractInstance.increment(addVal);
    await incReceipt.wait();
    num = await contractReader.getNumber();
    console.log(`ethers call contract update(inc ${addVal}), num is ${num}`);
}
