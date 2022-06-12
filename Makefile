build:
	cd contract && cargo build --target wasm32-unknown-unknown --release && cd ../ && \
	cp contract/target/wasm32-unknown-unknown/release/here_btc_bridge.wasm ./out/main.wasm

test:
	cd contract && RUST_BACKTRACE=1 cargo test && cd ..


deploy-dev:
	make build && \
	near dev-deploy


deploy-prod:
	make build && \
	NEAR_ENV=mainnet near deploy bridge.mydev.near


user-api:
	NEAR_ENV=mainnet near call bridge.mydev.near  create_request '{"account_id":"petr.near"}' --accountId mydev.near  --gas 242794783120800