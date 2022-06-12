use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, PanicOnDefault, Promise, PromiseOrValue,
    PromiseResult,
};
use serde::{Deserialize, Serialize};

macro_rules! owner {
    // using a ty token type for macthing datatypes passed to maccro
    ($self:expr) => {
        assert_eq!(
            $self.owner_id,
            env::predecessor_account_id(),
            "Avilible only for owner"
        );
    };
}

static WBTC_CONTRACT_ID: &str = "2260fac5e5542a773aa44fbcfedf7c193bc2c599.factory.bridge.near";
static HOUR: u64 = 3_600_000_000_000;
static MIN_BTC_AMOUN: u128 = 1000; // ~ 3 USD
static MAX_BTC_AMOUN: u128 = 1000000; // ~ 300 USD

static FT_TRANSFER_GAS: u64 = 25_000_000_000_000;
static CALLBACK_GAS: u64 = 25_000_000_000_000;
static TRANSACTION_COMISSION: u128 = 10_000_000_000_000_000_000_000;

#[ext_contract(ext_sp)]
pub trait WrappedBTC {
    fn ft_transfer(
        &self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> U128;
}

#[ext_contract(ext_self)]
pub trait MyContract {
    fn send_wbtc_callback(&self, request_id: String);
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SwapRequest {
    account_id: AccountId,
    target_btc_address: Option<String>,
    btc_amount: u128,
    deposit: u128,
    active_before_time: u64,
}

#[derive(Serialize, Deserialize)]
pub struct SwapRequestOut {
    target_btc_address: Option<String>,
    btc_amount: U128,
    active_before_time: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    available_btc_amount: u128,
    requests: UnorderedMap<String, SwapRequest>, // account id: (apy, last pay ts, min_balance)
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id: owner_id,
            available_btc_amount: 0,
            requests: UnorderedMap::new(b"a".to_vec()),
        }
    }

    pub fn change_owner(&mut self, new_owner_id: ValidAccountId) {
        owner!(self);
        self.owner_id = new_owner_id.into();
        env::log("Owner changed".as_bytes());
    }

    pub fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert_eq!(
            env::predecessor_account_id(),
            WBTC_CONTRACT_ID,
            "This is not wBTC contract"
        );

        self.available_btc_amount += amount.0;
        env::log(format!("Add {} wBTC from {}: {}", amount.0, sender_id, msg).as_bytes());
        PromiseOrValue::Value(U128(0))
    }

    // #[payable]
    pub fn create_request(&mut self, request_id: String, btc_amount: U128) {
        assert!(
            btc_amount.0 >= MIN_BTC_AMOUN,
            "Min transaction is 0.001 BTC"
        );

        // assert!(
        //     env::attached_deposit() >= btc_amount.0 * 30_000_000_000_000_000_000, // 5% from target btc amount
        //     "Not enought balance to create account, deposit shoud be more then 3% of requested balance"
        // );

        assert!(btc_amount.0 <= MAX_BTC_AMOUN, "Max transaction is 0.01 BTC");

        if let Some(new_balance) = self.available_btc_amount.checked_sub(btc_amount.0) {
            self.available_btc_amount = new_balance;
        } else {
            env::panic("The account doesn't have enough balance".as_bytes());
        }
        assert!(
            self.requests.get(&request_id).is_none(),
            "Request with this ID alreasy exist"
        );

        self.requests.insert(
            &request_id,
            &SwapRequest {
                account_id: env::predecessor_account_id(),
                btc_amount: btc_amount.0,
                active_before_time: 0,
                deposit: env::attached_deposit() - TRANSACTION_COMISSION,
                target_btc_address: None,
            },
        );
    }

    pub fn get_requests(&self) -> Vec<(String, SwapRequestOut)> {
        let mut resp: Vec<(String, SwapRequestOut)> = Vec::new();
        for el in self.requests.to_vec() {
            resp.push((
                el.0,
                SwapRequestOut {
                    target_btc_address: el.1.target_btc_address,
                    btc_amount: U128(el.1.btc_amount),
                    active_before_time: el.1.active_before_time,
                },
            ));
        }
        resp
    }


    pub fn get_request(&self, request_id: String) -> Option<SwapRequestOut> {
        let resp = self.requests.get(&request_id);
        if resp.is_none(){
            None
        }
        else{
            let sr = resp.unwrap();
            Some(SwapRequestOut {
                target_btc_address: sr.target_btc_address,
                btc_amount: U128(sr.btc_amount),
                active_before_time: sr.active_before_time,
            })
        }
    }

    pub fn available_btc(&self) -> U128 {
        U128(self.available_btc_amount)
    }

    pub fn activate_request(&mut self, request_id: String, target_btc_address: String) {
        owner!(self);
        let mut bid = self
            .requests
            .get(&request_id)
            .unwrap_or_else(|| env::panic("Request not found".as_bytes()));
        assert!(
            bid.target_btc_address.is_none(),
            "Request already activeted"
        );
        bid.target_btc_address = Some(target_btc_address);
        bid.active_before_time = env::block_timestamp() + HOUR;
        self.requests.insert(&request_id, &bid);
    }

    pub fn complete_request(&mut self, request_id: String) -> Promise {
        owner!(self);
        let bid = self
            .requests
            .get(&request_id)
            .unwrap_or_else(|| env::panic("Request not found".as_bytes()));

        let account_id = bid.account_id.clone();

        ext_sp::ft_transfer(
            bid.account_id,
            U128(bid.btc_amount),
            None,
            "".to_string(),
            &WBTC_CONTRACT_ID,
            1,
            FT_TRANSFER_GAS,
        )
        .then(ext_self::send_wbtc_callback(
            request_id,
            &env::current_account_id(),
            0,
            CALLBACK_GAS,
        ))
        .then(Promise::new(account_id).transfer(bid.deposit))
    }

    pub fn close_request(&mut self, request_id: String) {
        let bid = self
            .requests
            .get(&request_id)
            .unwrap_or_else(|| env::panic("Request not found".as_bytes()));

        if self.owner_id == env::predecessor_account_id() {
            assert!(
                bid.active_before_time < env::block_timestamp(),
                "U cant close request before {}",
                bid.active_before_time
            );
        } else {
            assert!(
                env::predecessor_account_id() == bid.account_id,
                "U cant close not ur request {}",
                bid.active_before_time
            );
        }

        self.requests.remove(&request_id);
        self.available_btc_amount += bid.btc_amount;
    }

    #[private]
    pub fn send_wbtc_callback(&mut self, request_id: String) {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");

        // handle the result from the cross contract call this method is a callback for
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic("oops!".as_bytes()),
            PromiseResult::Successful(_result) => {
                self.requests.remove(&request_id);
            }
        }
    }
}
