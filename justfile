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
    anchor deploy --provider.cluster devnet --provider.wallet ~/.config/solana/id.json

# Deploy Bounty to a specific RPC_URL
deploy RPC_URL:
    @echo "Deploying Bounty to RPC..."
    anchor deploy --provider.cluster {{RPC_URL}} --provider.wallet ~/.config/solana/id.json

# Recover fees in case deployment failed
recover_deploy:
    @echo "Recovering Bounty..."
    solana-keygen recover -o recover.json
    solana program close recover.json

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