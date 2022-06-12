BTC Bridge contract
=====

## How to use bridge

как получить NEAR за BTC
0. Init accounts
```
NEAR_ENV=mainnet near call 2260fac5e5542a773aa44fbcfedf7c193bc2c599.factory.bridge.near storage_deposit '{}' --gas 150000000000000 --deposit 0.1  --accountId bridge.mydev.near 

NEAR_ENV=mainnet near call v1.orderbook.near storage_deposit '{}' --gas 242794783120800 --deposit 0.01  --accountId mydev.near

NEAR_ENV=mainnet near call wrap.near near_deposit '{}'  --deposit 0.01  --gas 242794783120800 --accountId mydev.near
```
1. create a transfer request with a random `request_id` of 8 characters
```bash
NEAR_ENV=mainnet near call  bridge.mydev.near  create_request '{"request_id":"fh9032gh905", "btc_amount": "10000"}' --accountId mydev.near  --gas 242794783120800
```
2. Every 3s we check the requests 
```
NEAR_ENV=mainnet near view bridge.mydev.near get_request '{"request_id":"fh9032gh905"}'
```
3. as soon as `target_btc_address` comes in the response, show it
4. regularly check your balance in btc with the command
```
NEAR_ENV=mainnet near view 2260fac5e5542a773aa44fbcfedf7c193bc2c599.factory.bridge.near ft_balance_of '{"account_id":"mydev.near"}'  
```
5. balance is positive - it means bitcoin came, now you need to sell it to get NEAR
```
NEAR_ENV=mainnet near call a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.factory.bridge.near ft_transfer_call '{"receiver_id": "v1.orderbook.near", "amount": "1", "msg": "{market_id: \"2UmzUXYpaZg4vXfFVmD7r8mYUYkKEF19xpjLw7ygDUwp\", side: \"Buy\", amount_in: \"10\", min_amount_out: \"1\"}"}' --accountId mydev.near --depositYocto 1 --gas 300000000000000

```

6. Greate! We've got the nir, only it's a wrapped. Let's unwrapped him
```
NEAR_ENV=mainnet near call wrap.near near_withdraw '{"amount":"1000"}'  --depositYocto 1 --gas 242794783120800 --accountId mydev.near
```


-------

Mainnet contract ID: `bridge.mydev.near`


