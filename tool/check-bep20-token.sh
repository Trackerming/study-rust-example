#!/bin/bash

bsc_usd_contract_address="0x55d398326f99059fF775485246999027B3197955";
bsc_api_key="***";
bsc_base_url="https://api.bscscan.com/api";

eth_usd_contract_address="0xdac17f958d2ee523a2206206994597c13d831ec7";
eth_api_key="***";
eth_base_url="https://api.etherscan.io/api";

# 输入chain，和native or erc20 token对应的操作
# 参数分别对应 chain module action address 合约操作可选参数5
get_req_url() {
  if [ "$1" = "BSC" ]; then
    url="$bsc_base_url?module=$2&action=$3&tag=latest&apikey=$bsc_api_key&address=$4"
  elif [ "$1" = "ETH" ]; then
    url="$eth_base_url?module=$2&action=$3&tag=latest&apikey=$eth_api_key&address=$4"
  else
    echo "Error: Unsupported chain: $1"
    return 1
  fi

  if [ $# -eq 5 ]; then
    url="$url&contractaddress=$5"
  fi

  echo "$url"
}

# 参数分别为 url 币种精度
request_onchain_balance() {
  result=$(curl -X GET "$1")
  if [ $? -ne 0 ]; then
      echo "Error: Failed to make HTTP request"
      return 1
  fi
  # echo "$result"
  # value=$(echo "$result" | jq -r '.result | tonumber / 10^18')
  value=$(echo "$result" | sed -n 's/.*"result":"\([^"]*\).*/\1/p')
  value=$(echo "scale=$2; $value / 10^$2" | bc)
  # value=$(printf "%.0f" "$value")
  echo $value
}

# 参数1代表chain， BSC或者ETH，参数2代表精度
query_balance_by_chain() {
  sum=0
  if [ "$1" = "BSC" ]; then
    contract_address=$bsc_usd_contract_address
  elif [ "$1" = "ETH" ]; then
    contract_address=$eth_usd_contract_address
  fi
  # 循环遍历数组中的元素
  #for index in "${!addresses[@]}"; do
  local addrs=("${!3}")
  local bals=("${!4}")
  for index in "${!addrs[@]}"; do
      addr="${addrs[index]}"
      balance="${bals[index]}"
      # echo "$addr $balance"
      # 请求native token
      url=$(get_req_url "$1" "account" "balance" "$addr")
      native_value=$(request_onchain_balance "$url" 18)
      url=$(get_req_url "$1" "account" "tokenbalance" "$addr" "$contract_address")
      # echo "Generated URL: $url"
      # 请求数据
      value=$(request_onchain_balance "$url" $2)
      if [ "$(echo "$value == $balance" | bc)" -eq 1 ]; then
          echo "address: $addr "$1"-USD: $value; $1 chain native token: $native_value"
      else
          echo "address: $addr onchain balance: $value return balance: $balance; $1 chain native token: $native_value"
      fi
      sum=$(echo "$sum" + "$value" | bc)
      # echo "current sum: $sum"
      sleep 1
  done
  # 只能return整数
  # return "$sum"
  echo "contract balance sum: $sum"
}

# 定义数组，包含请求的参数
addresses=(
"0x9696f59E4d72E237BE84fFD425DCaD154Bf96976"
"0x1AB4973a48dc892Cd9971ECE8e01DcC7688f8F23"
)
balances=(
2500.00000000
23319761.069317
)

query_balance_by_chain "ETH" 6 "addresses[@]" "balances[@]"
sleep 2
query_balance_by_chain "BSC" 18 "addresses[@]" "balances[@]"
