# Autonomy Network

impermenant_loss_stop_loss on Solana blockchain with Raydium integration

**Devnet program id**: 2QGo9WwyXbFzyCnrob9XuLbEwqxmNhn4a58w4BxZBer5

## How to run

### Program

config to use devnet

```
anchor build
solana deploy <your-path-to-file.so>
```

copy your program id into PROGRAM_ID constant in tests/newVault.js

```
anchor test --skip-build --skip-deploy
```

## Program detail

There are 4 instructions

1. initialize_impermenant_loss_stop_loss
    - save min_change_factor, token a address, amount & token b address, amount
    - call raydium deposit with 0 to check if pair exists
2. owner_add_liquidity
    - transfer user's token a, b to contract to manage
    - call raydium add liquidity
3. owner_remove_liquidity
    - call raydium remove liquidity
    - transfer user's token a, b from contract back to user
3. anyone_remove_liquidity
    - allows anyone (our bot) to check if token a, b in liquidity pool have deviated from min_change_factor variable
    - if true, remove liquidity and transfer back to user, else revert
## Todo

- Need to update anchor version and corresponding compile time issues (using version 0.7.0 here while it should be 0.17.0)