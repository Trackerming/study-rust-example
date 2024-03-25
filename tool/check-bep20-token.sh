#!/bin/bash

bsc_usd_contract_address="0x55d398326f99059fF775485246999027B3197955";
bsc_api_key="apikey";
bsc_url="https://api.bscscan.com/api?module=account&action=tokenbalance&contractaddress=$bsc_usd_contract_address&tag=latest&apikey=$bsc_api_key&address=";

# 定义数组，包含请求的参数
addresses=(
"0x32d03F46BA2857c8E6A920aB3fed1F24d35D85d1"
"0xdad4FAF4c18Ec6892F1Ac4A552a20E5C5Fa26251"
)
balances=(
2500.00000000
39.7
)
sum=0
# 循环遍历数组中的元素
for index in "${!addresses[@]}"; do
    addr="${addresses[index]}"
    balance=${balances[index]}
    # 请求数据
    result=$(curl -X GET "$bsc_url$addr")
    # echo "$result"
    # value=$(echo "$result" | jq -r '.result | tonumber / 10^18')
    value=$(echo "$result" | sed -n 's/.*"result":"\([^"]*\).*/\1/p')
    value=$(echo "scale=18; $value / 10^18" | bc)
    # value=$(printf "%.0f" "$value")
    if [ "$(echo "$value == $balance" | bc)" -eq 1 ]; then
        echo "address: $addr BSC-USD: $value"
    else
        echo "address: $addr onchain balance: $value return balance: ${balances[index]}"
    fi
    sum=$(echo "$sum" + "$value" | bc)
    # echo "current sum: $sum"
    sleep 1
done
echo "addresses BSCUSD chain balance SUM = $sum"
