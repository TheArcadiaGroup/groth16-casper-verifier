use crate::utilities::get_current_time;
use ark_ec::bn::BnParameters;
use casper_engine_test_support::{
    DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT,
    DEFAULT_ACCOUNT_INITIAL_BALANCE, DEFAULT_ACCOUNT_PUBLIC_KEY, DEFAULT_AUCTION_DELAY,
    DEFAULT_GENESIS_CONFIG_HASH, DEFAULT_GENESIS_TIMESTAMP_MILLIS,
    DEFAULT_LOCKED_FUNDS_PERIOD_MILLIS, DEFAULT_PAYMENT, DEFAULT_PROPOSER_PUBLIC_KEY,
    DEFAULT_PROTOCOL_VERSION, DEFAULT_ROUND_SEIGNIORAGE_RATE, DEFAULT_SYSTEM_CONFIG,
    DEFAULT_UNBONDING_DELAY, DEFAULT_VALIDATOR_SLOTS, DEFAULT_WASM_CONFIG,
};
use casper_execution_engine::core::engine_state::{
    genesis::{ExecConfig, GenesisAccount},
    run_genesis_request::RunGenesisRequest,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{Bytes, FromBytes},
    runtime_args, CLTyped, ContractHash, Key, Motes, PublicKey, RuntimeArgs, SecretKey,
};
use rand::Rng;
use std::path::PathBuf;

// contains methods that can simulate a real-world deployment (storing the contract in the blockchain)
// and transactions to invoke the methods in the contract.
pub const VERIFIER_CONTRACT_KEY_NAME: &str = "Verifier";

pub struct Sender(pub AccountHash);
pub type Hash = [u8; 32];

pub struct Config {}

impl Config {
    /// Creates a vector of [`GenesisAccount`] out of a vector of [`PublicKey`].
    pub fn set_custom_accounts(public_keys: Vec<PublicKey>) -> Vec<GenesisAccount> {
        let mut genesis_accounts = Vec::new();

        // add default and proposer accounts.
        let genesis_account = GenesisAccount::account(
            DEFAULT_ACCOUNT_PUBLIC_KEY.clone(),
            Motes::new(DEFAULT_ACCOUNT_INITIAL_BALANCE.into()),
            None,
        );
        genesis_accounts.push(genesis_account);
        let proposer_account = GenesisAccount::account(
            DEFAULT_PROPOSER_PUBLIC_KEY.clone(),
            Motes::new(DEFAULT_ACCOUNT_INITIAL_BALANCE.into()),
            None,
        );
        genesis_accounts.push(proposer_account);

        // add custom accounts.
        for public_key in public_keys {
            let genesis_account = GenesisAccount::account(
                public_key.clone(),
                Motes::new(DEFAULT_ACCOUNT_INITIAL_BALANCE.into()),
                None,
            );
            genesis_accounts.push(genesis_account);
        }
        genesis_accounts
    }

    /// Creates an [`ExecConfig`] out of the given `genesis_accounts`
    /// and uses default values for the other params.
    pub fn set_custom_exec_config(genesis_accounts: Vec<GenesisAccount>) -> ExecConfig {
        ExecConfig::new(
            genesis_accounts,
            *DEFAULT_WASM_CONFIG,
            *DEFAULT_SYSTEM_CONFIG,
            DEFAULT_VALIDATOR_SLOTS,
            DEFAULT_AUCTION_DELAY,
            DEFAULT_LOCKED_FUNDS_PERIOD_MILLIS,
            DEFAULT_ROUND_SEIGNIORAGE_RATE,
            DEFAULT_UNBONDING_DELAY,
            DEFAULT_GENESIS_TIMESTAMP_MILLIS,
        )
    }

    /// Creates a new [`RunGenesisRequest`] given a custom [`ExecConfig`].
    pub fn set_custom_run_genesis_request(custom_exec_config: ExecConfig) -> RunGenesisRequest {
        RunGenesisRequest::new(
            *DEFAULT_GENESIS_CONFIG_HASH,
            *DEFAULT_PROTOCOL_VERSION,
            custom_exec_config,
        )
    }

    /// Deploys a contract and returns the `contract_hash` and the updated `builder`.
    pub fn deploy_contract(
        mut builder: InMemoryWasmTestBuilder,
        session_code: PathBuf,
        session_args: RuntimeArgs,
        deployer: PublicKey,
        contract_hash_key: String,
    ) -> (InMemoryWasmTestBuilder, Hash) {
        let mut rng = rand::thread_rng();

        let deploy_item = DeployItemBuilder::new()
            // .with_payment_bytes(module_bytes, args)
            .with_empty_payment_bytes(runtime_args! {
                ARG_AMOUNT => *DEFAULT_PAYMENT
            })
            .with_session_code(session_code, session_args)
            .with_deploy_hash(rng.gen())
            .with_authorization_keys(&[deployer.to_account_hash()])
            .with_address(deployer.to_account_hash())
            .build();

        // prepare the execute request.
        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item)
            .with_block_time(get_current_time())
            .build();

        // pre-assertion before the contract deployment.
        let contract_hash = builder.query(
            None,
            Key::Account(deployer.to_account_hash()),
            &[contract_hash_key.clone()],
        );

        assert!(contract_hash.is_err());

        // deploy the contract.
        builder.exec(execute_request).commit().expect_success();

        // retrieving hashes & post-assertions after the contract deployment.
        let contract_hash = builder
            .get_account(deployer.to_account_hash())
            .expect("should have account")
            .named_keys()
            .get(&contract_hash_key)
            .and_then(|key| key.into_hash())
            .map(ContractHash::new)
            .expect("should have contract hash")
            .value();

        assert_ne!(contract_hash, [0u8; 32]);

        (builder, contract_hash)
    }
}

pub struct Verifier {
    pub builder: InMemoryWasmTestBuilder,
    pub hash: Hash,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
}

impl Verifier {
    pub fn deployed() -> Verifier {
        // ====================== ACCOUNTS SETUP ======================
        let ali = PublicKey::from(&SecretKey::ed25519_from_bytes([3u8; 32]).unwrap());
        let bob = PublicKey::from(&SecretKey::ed25519_from_bytes([6u8; 32]).unwrap());
        let joe = PublicKey::from(&SecretKey::ed25519_from_bytes([9u8; 32]).unwrap());

        // ====================== BLOCKCHAIN SETUP ======================
        // create our WasmBuilder.
        let mut builder = InMemoryWasmTestBuilder::default();

        // initialize the blockchain network to get our first block.

        // implement custom accounts.
        let genesis_accounts: Vec<GenesisAccount> =
            Config::set_custom_accounts(vec![ali.clone(), bob.clone()]);

        // implement custom exec config.
        let custom_exec_config: ExecConfig = Config::set_custom_exec_config(genesis_accounts);

        // implement custom run genesis request.
        let custom_run_genesis_request: RunGenesisRequest =
            Config::set_custom_run_genesis_request(custom_exec_config);

        // run genesis request with the custom exec config.
        builder.run_genesis(&custom_run_genesis_request).commit();

        // ====================== CONTRACT DEPLOYMENT ======================
        let session_code = PathBuf::from("contract.wasm");
        let session_args = runtime_args! {};

        let (builder, hash) = Config::deploy_contract(
            builder,
            session_code,
            session_args,
            ali.clone(),
            VERIFIER_CONTRACT_KEY_NAME.to_string(),
        );

        // ====================== FUNCTION RETURN ======================
        Verifier {
            builder,
            hash,
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
            joe: joe.to_account_hash(),
        }
    }

    /// query a contract's named key.
    fn _query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self.builder.query(
            None,
            Key::Account(self.ali),
            &[VERIFIER_CONTRACT_KEY_NAME.to_string(), name.to_string()],
        ) {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .as_cl_value()
                    .expect("should be cl value.")
                    .clone()
                    .into_t()
                    .expect("should have the correct type.");
                Some(value)
            }
        }
    }

    /// call a contract's specific entry point.
    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;

        // prepare the deploy item.
        let deploy_item = DeployItemBuilder::new()
            // .with_payment_bytes(module_bytes, args)
            .with_empty_payment_bytes(runtime_args! {
                ARG_AMOUNT => *DEFAULT_PAYMENT
            })
            .with_stored_session_hash(self.hash.into(), method, args)
            .with_authorization_keys(&[address])
            .with_address(address)
            .build();

        // prepare the execute request.
        // we can use .with_block_time() when setting the execute request.
        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();

        // executes the execute_request.
        self.builder.exec(execute_request).commit().expect_success();
    }

    pub fn gamma_miller_loop(&mut self, prepared_input: Vec<u8>, key: AccountHash) {
        let mut j: u8 = 0;
        for i in (1..ark_bn254::Parameters::ATE_LOOP_COUNT.len()).rev() {
            self._gamma_miller_loop(i as u8, j, prepared_input.clone(), Sender(key));

            j += 1;
            if ark_bn254::Parameters::ATE_LOOP_COUNT[i - 1] == 1
                || ark_bn254::Parameters::ATE_LOOP_COUNT[i - 1] == -1
            {
                j += 1;
            }
        }

        self._gamma_miller_loop(0, j, prepared_input, Sender(key));
    }

    pub fn delta_miller_loop(&mut self, proof_c: Vec<u8>, key: AccountHash) {
        let mut j: u8 = 0;
        for i in (1..ark_bn254::Parameters::ATE_LOOP_COUNT.len()).rev() {
            self._delta_miller_loop(i as u8, j, proof_c.clone(), Sender(key));

            j += 1;
            if ark_bn254::Parameters::ATE_LOOP_COUNT[i - 1] == 1
                || ark_bn254::Parameters::ATE_LOOP_COUNT[i - 1] == -1
            {
                j += 1;
            }
        }

        self._delta_miller_loop(0, j, proof_c, Sender(key));
    }

    pub fn final_exponentiation(&mut self, qap: Vec<u8>) {
        let gamma_key = "gamma".to_string();
        let delta_key = "delta".to_string();
        let final_key = "final".to_string();
        // first, create account for y0..y16
        let mut final_keys = vec![];
        for i in 0..17 {
            final_keys.push(["y".to_string(), i.to_string()].join(""));
        }

        // prepare_final_data
        self._final_exponentiation(
            0,
            0,
            qap,
            vec![gamma_key, delta_key, final_key.clone()],
            Sender(self.joe),
        );

        // easy_part1
        self._final_exponentiation(0, 0, vec![], vec![final_key.clone()], Sender(self.joe));

        // easy_part2
        self._final_exponentiation(0, 0, vec![], vec![final_key.clone()], Sender(self.joe));

        // hard_part_y0
        for i in 0..63 {
            self._final_exponentiation(
                0,
                i,
                vec![],
                vec![final_key.clone(), final_keys[0].clone()],
                Sender(self.joe),
            );
        }

        // hard_part_y1
        self._final_exponentiation(
            0,
            64,
            vec![],
            vec![final_keys[0].clone(), final_keys[1].clone()],
            Sender(self.joe),
        );

        // hard_part_y3
        self._final_exponentiation(
            0,
            0,
            vec![],
            vec![final_keys[0].clone(), final_keys[3].clone()],
            Sender(self.joe),
        );

        // hard_part_y4
        for i in 0..63 {
            self._final_exponentiation(
                0,
                i,
                vec![],
                vec![final_keys[3].clone(), final_keys[4].clone()],
                Sender(self.joe),
            );
        }

        // hard_part_y6
        for i in 0..63 {
            self._final_exponentiation(
                0,
                i,
                vec![],
                vec![final_keys[4].clone(), final_keys[6].clone()],
                Sender(self.joe),
            );
        }

        // hard_part_y8
        self._final_exponentiation(
            0,
            0,
            vec![],
            vec![
                final_keys[3].clone(),
                final_keys[4].clone(),
                final_keys[6].clone(),
                final_keys[8].clone(),
            ],
            Sender(self.joe),
        );

        // hard_part_y9
        self._final_exponentiation(
            0,
            0,
            vec![],
            vec![
                final_keys[1].clone(),
                final_keys[8].clone(),
                final_keys[9].clone(),
            ],
            Sender(self.joe),
        );

        // hard_part_y11
        self._final_exponentiation(
            0,
            0,
            vec![],
            vec![
                final_keys[4].clone(),
                final_keys[8].clone(),
                final_key.clone(),
                final_keys[11].clone(),
            ],
            Sender(self.joe),
        );

        // hard_part_y13
        self._final_exponentiation(
            0,
            0,
            vec![],
            vec![
                final_keys[9].clone(),
                final_keys[11].clone(),
                final_keys[13].clone(),
            ],
            Sender(self.joe),
        );

        // hard_part_y14
        self._final_exponentiation(
            0,
            0,
            vec![],
            vec![
                final_keys[8].clone(),
                final_keys[13].clone(),
                final_keys[14].clone(),
            ],
            Sender(self.joe),
        );

        // hard_part_y15
        self._final_exponentiation(
            0,
            0,
            vec![],
            vec![
                final_keys[9].clone(),
                final_key.clone(),
                final_keys[15].clone(),
            ],
            Sender(self.joe),
        );

        // hard_part_y16
        self._final_exponentiation(
            0,
            0,
            vec![],
            vec![final_keys[14].clone(), final_keys[15].clone()],
            Sender(self.joe),
        );
    }

    fn _gamma_miller_loop(&mut self, i: u8, j: u8, prepared_input: Vec<u8>, sender: Sender) {
        println!("{} {}", i, j);
        self.call(
            sender,
            "gamma_miller_loop",
            runtime_args! {
                "i" => i,
                "j" => j,
                "input" => Bytes::from(prepared_input)
            },
        );
    }

    fn _delta_miller_loop(&mut self, i: u8, j: u8, proof_c: Vec<u8>, sender: Sender) {
        self.call(
            sender,
            "delta_miller_loop",
            runtime_args! {
                "i" => i,
                "j" => j,
                "input" => Bytes::from(proof_c)
            },
        );
    }

    fn _final_exponentiation(
        &mut self,
        i: u8,
        j: u8,
        qap: Vec<u8>,
        keys: Vec<String>,
        sender: Sender,
    ) {
        self.call(
            sender,
            "delta_miller_loop",
            runtime_args! {
                "i" => i,
                "j" => j,
                "input" => Bytes::from(qap),
                "keys" => keys
            },
        );
    }
}
