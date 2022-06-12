NEAR_ENV=mainnet near call wrap.near near_deposit '{}'  --deposit 10  --gas 242794783120800 --accountId mydev.near
NEAR_ENV=mainnet near call wrap.near near_withdraw '{"amount":"1000"}'  --depositYocto 1 --gas 242794783120800 --accountId mydev.near

NEAR_ENV=mainnet near call wrap.near ft_transfer_call '{"receiver_id":"v2.ref-finance.near", "amount": "10000000000000000000000000", "msg": "{\"force\":0,\"actions\":[{\"pool_id\":974,\"token_in\":\"wrap.near\",\"token_out\":\"2260fac5e5542a773aa44fbcfedf7c193bc2c599.factory.bridge.near\",\"amount_in\":\"1000000000000000000000000\",\"min_amount_out\":\"0\"}]}" }'  --gas 242794783120800  --depositYocto 1 --accountId mydev.near

NEAR_ENV=mainnet near view v2.ref-finance.near get_pool '{"pool_id":974}' 

NEAR_ENV=mainnet near call v2.ref-finance.near swap '{"actions":[{"pool_id": 974, "token_in":"wrap.near", "token_out": "2260fac5e5542a773aa44fbcfedf7c193bc2c599.factory.bridge.near", "min_amount_out": "0", "amount_in": "1000000000000000000000000"}]}' --gas 242794783120800 --accountId mydev.near

NEAR_ENV=mainnet near call v2.ref-finance.near register_tokens '{"token_ids":["wrap.near", "2260fac5e5542a773aa44fbcfedf7c193bc2c599.factory.bridge.near"]}' --gas 242794783120800  --depositYocto 1 --accountId mydev.near

    NEAR_ENV=mainnet near call v2.ref-finance.near storage_deposit '{}' --gas 242794783120800 --deposit 0.01  --accountId mydev.near 

NEAR_ENV=mainnet near call 2260fac5e5542a773aa44fbcfedf7c193bc2c599.factory.bridge.near storage_deposit '{}' --gas 242794783120800 --deposit 0.01  --accountId mydev.near 


-----

near call ${AID}  new '{"owner_id":"petr4.testnet"}' --accountId petr4.testnet  --gas 242794783120800 

near call ${AID}  create_request '{"request_id":"12", "btc_amount": "100000"}' --accountId petr4.testnet  --gas 242794783120800 
near call ${AID}  activate_request '{"request_id":"12", "target_btc_adress": "8gd89gf89289f8g289"}' --accountId petr4.testnet  --gas 242794783120800 
near call ${AID}  close_request '{"request_id":"12"}' --accountId petr4.testnet  --gas 242794783120800 
near view ${AID}  get_requests '{}'

bridge.mydev.near
NEAR_ENV=mainnet near depoly bridge.mydev.near 
NEAR_ENV=mainnet near view bridge.mydev.near available_btc ''
NEAR_ENV=mainnet near view bridge.mydev.near get_requests ''
NEAR_ENV=mainnet near view bridge.mydev.near get_request '{"request_id":"fh9032gh905"}'
NEAR_ENV=mainnet near call  bridge.mydev.near  new '{"owner_id":"mydev.near"}' --accountId mydev.near  --gas 242794783120800 

NEAR_ENV=mainnet near call  bridge.mydev.near  create_request '{"request_id":"fh9032gh905", "btc_amount": "10000"}' --accountId mydev.near  --gas 242794783120800 
NEAR_ENV=mainnet near call  bridge.mydev.near  activate_request '{"request_id":"1", "target_btc_address": "bc1q7gx8stgvhejcc7ppwfndarthtegqkhca0d8zwm"}' --accountId mydev.near  --gas 242794783120800 

NEAR_ENV=mainnet near call  bridge.mydev.near  complete_request '{"request_id":"1"}' --accountId mydev.near  --gas 242794783120800 
NEAR_ENV=mainnet near call  bridge.mydev.near  close_request '{"request_id":"1"}' --accountId mydev.near  --gas 242794783120800 

NEAR_ENV=mainnet near call 2260fac5e5542a773aa44fbcfedf7c193bc2c599.factory.bridge.near ft_transfer_call '{"receiver_id":"bridge.mydev.near", "amount": "6903", "msg": "" }'  --gas 242794783120800  --depositYocto 1 --accountId mydev.near
NEAR_ENV=mainnet near call 2260fac5e5542a773aa44fbcfedf7c193bc2c599.factory.bridge.near ft_transfer_call '{"receiver_id":"bridge.mydev.near", "amount": "27775", "msg": "" }'  --gas 242794783120800  --depositYocto 1 --accountId bridge.mydev.near
NEAR_ENV=mainnet near view 2260fac5e5542a773aa44fbcfedf7c193bc2c599.factory.bridge.near ft_balance_of '{"account_id":"bridge.mydev.near"}'  
NEAR_ENV=mainnet near call 2260fac5e5542a773aa44fbcfedf7c193bc2c599.factory.bridge.near storage_deposit '{}' --gas 150000000000000 --deposit 0.1  --accountId bridge.mydev.near 


NEAR_ENV=mainnet near call 2260fac5e5542a773aa44fbcfedf7c193bc2c599.factory.bridge.near ft_transfer_call '{"receiver_id":"v2.ref-finance.near", "amount": "10000", "msg": "{\"force\":0,\"actions\":[{\"pool_id\":974,\"token_in\":\"2260fac5e5542a773aa44fbcfedf7c193bc2c599.factory.bridge.near\",\"token_out\":\"wrap.near\",\"amount_in\":\"10000\",\"min_amount_out\":\"0\"}]}" }'  --gas 242794783120800  --depositYocto 1 --accountId mydev.near
