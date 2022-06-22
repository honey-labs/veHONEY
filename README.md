# veHoney vesting program
## Mint authority instructions
```bash
yarn install 
```
```bash
anchor --version
```
anchor-cli 0.22.1

Make sure `anchor --version` returns version 0.22.1

### Compile Solana program to get types
```bash
anchor test
```

### Point correct private key
- In `line 9` point to the mainnet RPC url on the discord.
- In `line 94` of the `test/mainnet-script.ts` point that to your ledger's exported private key file.
- In `line 123`of point to the `base_locker.json`
- In line `line 232` point to the Public Key of the stake pool. 

### Run migration script
```bash
export ANCHOR_PROVIDER_URL=https://polished-aged-star.solana-mainnet.quiknode.pro/d28283c8e1dece7b2a9317d6fa4dfcb76ee7f4e8/
export ANCHOR_WALLET=/home/<replace_with_user>/config/solana/id.json
ts-node test/mainnet-script.ts
```