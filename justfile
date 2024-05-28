rpc_url_devnet := env_var('RPC_URL_DEVNET')


# Build the bounty sdk 
build_sdk:
    @echo "Building Bounty SDK..."
    anchor run build_sdk

# Build the bounty program
build_program:
    @echo "Building Bounty..."
    anchor build 

# Build Bounty project
build: build_program build_sdk



# Test project using the sdk
test: build_sdk
    @echo "Testing Bounty SDK..."
    anchor test

# Deploy project to devnet
deploy_devnet:
    @echo "Deploying Bounty to devnet ..."
    anchor deploy --provider.cluster {{rpc_url_devnet}} --provider.wallet ~/.config/solana/id.json

upgrade_devnet:
    @echo "Deploying Bounty to devnet ..."
    anchor upgrade --provider.cluster {{rpc_url_devnet}} --provider.wallet ~/.config/solana/id.json --program-id 5DncffMLMaNXq9rLHa3B6UJpuo6XQinrJ1C8sx9JZD9w target/deploy/bounty.so


# Deploy Bounty to a specific RPC_URL
deploy RPC_URL:
    @echo "Deploying Bounty to RPC..."
    anchor deploy --provider.cluster {{RPC_URL}} --provider.wallet ~/.config/solana/id.json

# Recover fees in case deployment failed
recover_deploy_devnet:
    @echo "Recovering Bounty..."
    solana-keygen recover -o recover.json --force
    solana program close recover.json -u devnet

# Grind for keypair starting with PREFIX
grind_key PREFIX:
    @echo "Grinding for keypair starting with $(PREFIX)..."
    solana-keygen grind --starts-with {{PREFIX}}:1 --ignore-case

# Generate local keypair
generate_keypair NAME:
    @echo "Generating keypair..."
    solana-keygen new -o ~/.config/solana/{{NAME}}.json --no-bip39-passphrase

# Get program keypair address
program_address:
    @echo "Checking program address..."
    solana address --keypair target/deploy/bounty-keypair.json

airdrop: 
    @echo "Airdropping..."
    solana airdrop -u devnet -k ~/.config/solana/id.json 1


balance: 
    @echo "Checking balance..."
    solana balance

# Release SDK
release_sdk:
    @echo "Releasing Bounty..."
    yarn run release:sdk