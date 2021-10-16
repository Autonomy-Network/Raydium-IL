const anchor = require("@project-serum/anchor");
const { createMint } = require("@project-serum/common");
const spl = require("@solana/spl-token");
const { struct } = require("buffer-layout");
const { publicKey, u64 } = require("@project-serum/borsh");
const assert = require("assert");
const {
  mintToAccount,
  getTokenAccount,
  createTokenAccount,
} = require("./utils");
const idl = require("../target/idl/impermenant_loss_stop_loss.json");

const PROGRAM_ID = "8qi2yHnen1sULooCmigYr9KPm5csrbg3hBsZdAs9CPr8";

const RAYDIUM_STAKING_PROGRAM_ID =
  "EcLzTrNg9V7qhcdyXDe2qjtPkiGzDM2UbdRaeaadU5r2";
const RAYDIUM_POOL_ID = "2Bsexc5j6vk4r9RhBYz2ufPrRWhumXQk6efXucqUKsyr";
const RAYDIUM_POOL_AUTHORITY = "BxAtWJ4g6xguPsR9xNvXTK7EjuzwiKNbmKbhoXDZ3EsY";
const RAYDIUM_LP_MINT_ADDRESS = "14Wp3dxYTQpRMMz3AW7f2XGBTdaBrf1qb2NKjAN3Tb13";

const RAYDIUM_LP_VAULT_ADDRESS = "83BEhzv7eV4HeJuuPtYmHkhTjZEpNpK83mHnHfX5Krwj";
const RAYDIUM_REWARD_A = "HVtAJ1uRiWJ7tNU9uqAzpPv14B3fN9SVEW9G4PtM77Ci";
const RAYDIUM_REWARD_B = "39Ea6rMGGrsNmEsYToqQfEyNSqv7hcUJa646qBYLY4yq";

const RAYDIUM_AMM_PROGRAM_ID = "9rpQHSyFVM1dkkHFQ2TtTzPEW7DVmEyPmN8wVniqJtuC";
const RAYDIUM_AMM_ID = "HeD1cekRWUNR25dcvW8c9bAHeKbr1r7qKEhv7pEegr4f";
const RAYDIUM_AMM_AUTHORITY = "DhVpojXMTbZMuTaCgiiaFU7U8GvEEhnYo4G9BUdiEYGh";
const RAYDIUM_AMM_TOKEN_ACCOUNT =
  "3qbeXHwh9Sz4zabJxbxvYGJc57DZHrFgYMCWnaeNJENT";
const RAYDIUM_AMM_TOKEN_ACCOUNT_B =
  "FrGPG5D4JZVF5ger7xSChFVFL8M9kACJckzyCz8tVowz";
const RAYDIUM_AMM_OPEN_ORDERS = "HboQAt9BXyejnh6SzdDNTx4WELMtRRPCr7pRSLpAW7Eq";
const RAYDIUM_AMM_TARGET_ORDERS =
  "6TzAjFPVZVMjbET8vUSk35J9U2dEWFCrnbHogsejRE5h";
const SERUM_MARKET = "3tsrPhKrWHWMB8RiPaqNxJ8GnBhZnDqL4wcu5EAMFeBe";

const USER_LP_TOKEN_ACC = "3zeto42npEtrmkpmpmBeUfz2ZdUN42UA2LhFHnxEHk6u";
const USER_TOKEN_A_ACC = "AzrCx5xaxSRRw35P7P6psqW1mB8SmuqQc5USiXAdA1eE";
const USER_TOKEN_B_ACC = "2QxHHmuKLAGsZLqWtD995ewMtYV3ATKWXDnMMM2VGeev";

const TOKEN_A_MINT = "BEcGFQK1T1tSu3kvHC17cyCkQ5dvXqAJ7ExB2bb5Do7a";
const TOKEN_B_MINT = "FSRvxBNrQWX2Fy2qvKMLL3ryEdRtE3PUTZBcdKwASZTU";
const USER_STAKE_INFO_ACCOUNT_LAYOUT_V4 = struct([
  u64("state"),
  publicKey("poolId"),
  publicKey("stakerOwner"),
  u64("depositBalance"),
  u64("rewardDebt"),
  u64("rewardDebtB"),
]);

describe("test initialize, owner_add_liquidity", () => {
  const url = "https://api.devnet.solana.com";
  const preflightCommitment = "recent";
  const connection = new anchor.web3.Connection(url, preflightCommitment);
  const wallet = anchor.Wallet.local();

  const provider = new anchor.Provider(connection, wallet, {
    preflightCommitment,
    commitment: "recent",
  });

  anchor.setProvider(provider);
  const program = new anchor.Program(
    idl,
    PROGRAM_ID,
    provider
  );
  const RAYDIUM_PROGRAM_ID = "EcLzTrNg9V7qhcdyXDe2qjtPkiGzDM2UbdRaeaadU5r2";
  const POOL_ID = "2Bsexc5j6vk4r9RhBYz2ufPrRWhumXQk6efXucqUKsyr";
  const POOL_AUTHORITY = "BxAtWJ4g6xguPsR9xNvXTK7EjuzwiKNbmKbhoXDZ3EsY";
  const LP_MINT_ADDRESS = "14Wp3dxYTQpRMMz3AW7f2XGBTdaBrf1qb2NKjAN3Tb13";
  const lpTokenInstance = new spl.Token(
    provider.connection,
    LP_MINT_ADDRESS,
    spl.TOKEN_PROGRAM_ID,
    provider.wallet.payer
  );
  const RAYDIUM_LP_VAULT_ADDRESS =
    "83BEhzv7eV4HeJuuPtYmHkhTjZEpNpK83mHnHfX5Krwj";
  const RAYDIUM_REWARD_A = "HVtAJ1uRiWJ7tNU9uqAzpPv14B3fN9SVEW9G4PtM77Ci";
  const RAYDIUM_REWARD_B = "39Ea6rMGGrsNmEsYToqQfEyNSqv7hcUJa646qBYLY4yq";
  const USER_LP_TOKEN_ACC = "3UEbdr8S2AGUVJA2E36vtL86qnKfWjhLRiQy6fKysVz2";
  const USER_TOKEN_A_ACC = "AzrCx5xaxSRRw35P7P6psqW1mB8SmuqQc5USiXAdA1eE";
  const USER_TOKEN_B_ACC = "2QxHHmuKLAGsZLqWtD995ewMtYV3ATKWXDnMMM2VGeev";

  const TOKEN_A_MINT = "BEcGFQK1T1tSu3kvHC17cyCkQ5dvXqAJ7ExB2bb5Do7a";
  const tokenAInstance = new spl.Token(
    provider.connection,
    TOKEN_A_MINT,
    spl.TOKEN_PROGRAM_ID,
    provider.wallet.payer
  );
  const TOKEN_B_MINT = "FSRvxBNrQWX2Fy2qvKMLL3ryEdRtE3PUTZBcdKwASZTU";
  const tokenBInstance = new spl.Token(
    provider.connection,
    TOKEN_B_MINT,
    spl.TOKEN_PROGRAM_ID,
    provider.wallet.payer
  );
  const USER_STAKE_INFO_ACCOUNT_LAYOUT_V4 = struct([
    u64("state"),
    publicKey("poolId"),
    publicKey("stakerOwner"),
    u64("depositBalance"),
    u64("rewardDebt"),
    u64("rewardDebtB"),
  ]);

  let vaultAccount;
  let vaultSigner;
  let vaultTokenMintAddress;
  let vaultUserInfoAccount;
  let vaultLpTokenAccount;
  let vaultRewardTokenAccount;
  let vaultRewardTokenAccountB;

  let userVaultTokenAccount;
  let userSigner = provider.wallet.publicKey;
  // console.log("here!", tokenAInstance);
  if (!!program) {
    let programPubKey = program.programId;
    it("should initialize vault", async () => {
      console.log(programPubKey);
      vaultAccount = anchor.web3.Keypair.generate();
      if (!!vaultAccount) {
        const [_vaultSigner, nonce] =
          await anchor.web3.PublicKey.findProgramAddress(
            [vaultAccount.publicKey.toBuffer()],
            programPubKey
          );
        if (!!_vaultSigner) {
          vaultSigner = _vaultSigner;
        }
        vaultTokenMintAddress = await spl.Token.createMint(
          provider.connection,
          provider.wallet.payer,
          vaultSigner,
          provider.wallet.publicKey,
          6,
          spl.TOKEN_PROGRAM_ID
        );
        if (!!vaultTokenMintAddress) {
        vaultUserInfoAccount = anchor.web3.Keypair.generate();
        } 
        if (!!vaultUserInfoAccount) {
        vaultLpTokenAccount = await lpTokenInstance.createAccount(vaultSigner);
        }
        if (!!vaultLpTokenAccount) {
        vaultRewardTokenAccount = await tokenAInstance.createAccount(
          vaultSigner
        );
        }
        if (!!vaultRewardTokenAccount) {
        vaultRewardTokenAccountB = await tokenBInstance.createAccount(
          vaultSigner
        );
        }
        const createUserInfoAccountIx = anchor.web3.SystemProgram.createAccount(
          {
            fromPubkey: provider.wallet.publicKey,
            newAccountPubkey: vaultUserInfoAccount.publicKey,
            space: USER_STAKE_INFO_ACCOUNT_LAYOUT_V4.span,
            lamports:
              await provider.connection.getMinimumBalanceForRentExemption(
                USER_STAKE_INFO_ACCOUNT_LAYOUT_V4.span
              ),
            programId: new anchor.web3.PublicKey(RAYDIUM_PROGRAM_ID),
          }
        );
        anchor.setProvider(provider);
        let userVaultTokenAccount;
        let userSigner = provider.wallet.publicKey;
        console.log('eeee')
        const txSig = await program.rpc.initialize_impermenant_loss_stop_loss(nonce, 10, {
          accounts: {
            vaultAccount: vaultAccount.publicKey,
            vaultSigner,
            vaultTokenMintAddress: vaultTokenMintAddress.publicKey,
            vaultUserInfoAccount: vaultUserInfoAccount.publicKey,
            vaultLpTokenAccount,
            vaultRewardTokenAccount,
            vaultRewardTokenAccountB,
            raydiumStakeProgram: RAYDIUM_STAKING_PROGRAM_ID,
            raydiumPoolId: RAYDIUM_POOL_ID,
            raydiumPoolAuthority: RAYDIUM_POOL_AUTHORITY,
            raydiumLpTokenAccount: RAYDIUM_LP_VAULT_ADDRESS,
            raydiumRewardTokenAccount: RAYDIUM_REWARD_A,
            raydiumRewardTokenAccountB: RAYDIUM_REWARD_B,
            tokenProgram: spl.TOKEN_PROGRAM_ID,
            clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          },
          signers: [vaultAccount, vaultUserInfoAccount],
          instructions: [
            await program.account.vaultAccount.createInstruction(vaultAccount),
            createUserInfoAccountIx,
          ],
        });
        if (!!txSig) {
        console.log(txSig);

        const acc = await program.account.vaultAccount.fetch(
          vaultAccount.publicKey
        );
        if (!!acc) {
        assert.strictEqual(acc.nonce, nonce, "nonce not equal");
        }
        }
      }
    })
  }

  // it("should provide lp", async () => {
  //   // await tokenAInstance.transfer(USER_TOKEN_A_ACC, vaultRewardTokenAccount, userSigner, [], 10000);
  //   // await tokenBInstance.transfer(USER_TOKEN_B_ACC, vaultRewardTokenAccountB, userSigner, [], 10000);
  //   const txSig = await program.rpc.provideLiquidity(
  //     new anchor.BN(9000),
  //     new anchor.BN(9000),
  //     {
  //       accounts: {
  //         vaultAccount: vaultAccount.publicKey,
  //         vaultSigner,
  //         vaultLpTokenAccount,
  //         vaultRewardTokenAccount,
  //         vaultRewardTokenAccountB,
  //         // raydium
  //         raydiumAmmProgram: RAYDIUM_AMM_PROGRAM_ID,
  //         raydiumAmmId: RAYDIUM_AMM_ID,
  //         raydiumAmmAuthority: RAYDIUM_AMM_AUTHORITY,
  //         raydiumAmmOpenOrders: RAYDIUM_AMM_OPEN_ORDERS,
  //         raydiumAmmTargetOrders: RAYDIUM_AMM_TARGET_ORDERS,
  //         raydiumLpTokenMintAddress: RAYDIUM_LP_MINT_ADDRESS,
  //         raydiumRewardTokenAccount: RAYDIUM_AMM_TOKEN_ACCOUNT,
  //         raydiumRewardTokenAccountB: RAYDIUM_AMM_TOKEN_ACCOUNT_B,
  //         serumMarket: SERUM_MARKET,
  //         tokenProgram: spl.TOKEN_PROGRAM_ID,
  //       },
  //     }
  //   );
  //   console.log(txSig);
  // });
});
