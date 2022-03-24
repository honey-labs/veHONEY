# veHoney vesting program
This repository contains
## How to deploy to devnet
### Solana environment configuration and wallet key recovery
Create a new wallet and save the private key
```bash
solana-keygen new --force
```
Set the wallet to your solana config
```bash
solana config set -k ~/.config/solana/id.json
```
Set devnet to your solana config 
```bash
solana config set --url devnet
solana config set --url https://fragrant-silent-dream.solana-devnet.quiknode.pro/68267c5fd3b1e070262537335e454c72acd23ad5/
```
Now, check if your solana config is configured right, your output to ```solana config get ``` should look like the following:
```bash
➜  veHONEY git:(main) ✗ solana config get
Config File: /home/g69420/.config/solana/cli/config.yml
RPC URL: https://api.devnet.solana.com 
WebSocket URL: wss://api.devnet.solana.com/ (computed)
Keypair Path: /home/g69420/.config/solana/id.json 
Commitment: confirmed 
```

Again, make sure you copy the 12-word recovery seed words outputted when creating the wallet. This is the only wallet that has 

### Build your program

Make sure the ```target``` directory is removed.Since we want to generate a new IDL. 

Then run
```bash
anchor build 
```
Now get the public key from the program in order to deploy
```bash
$ solana address -k "target/deploy/ve_honey-keypair.json"
2GDyvBn4XzhYNwDYAW1B2W6bv17r1ZLBUq74kLeo12nR
```
Save this address to input into the ```Anchor.toml``` and ```ve_honey/src/lib.rs``` 

### Change Anchor.toml for devnet
```toml
[features]
seeds = false
<!-- This was previously programs.localnet -->
<!-- This was previously programs.localnet -->

[programs.devnet]
ve_honey = "2GDyvBn4XzhYNwDYAW1B2W6bv17r1ZLBUq74kLeo12nR"

[registry]
url = "https://anchor.projectserum.com"

<!-- This was previously cluster = "localnet" -->

[provider]
cluster = "devnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"

```

### Change declare_id from rust code to new program public key
Inside the ```ve_honey/src/lib.rs```. Replace the declare id 
```rust
declare_id!("4V68qajTiVHm3Pm9fQoV8D4tEYBmq3a34R9NV5TymLr7");
```
With the new address
```rust
declare_id!("2GDyvBn4XzhYNwDYAW1B2W6bv17r1ZLBUq74kLeo12nR");
```

### Do the same for the stake program
```bash
solana address -k target/deploy/stake-keypair.json
anchor build
```
Copy the address in the output of the command

## Deploy
```bash
anchor deploy
```