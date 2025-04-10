import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";

describe("vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.vault as Program<Vault>;

  console.log("Program ID (Contract ID):", program.programId.toBase58());
  console.log("Wallet Address:", provider.wallet.publicKey.toBase58());

  const vaultState = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state"), provider.wallet.publicKey.toBytes()],
    program.programId
  )[0];

  const vault = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vaultState.toBytes()],
    program.programId
  )[0];

  const logVaultInfo = async () => {
    const vaultAccountData = await program.account.vaultState.fetchNullable(vaultState);
    console.log("Vault State Data (parsed):", vaultAccountData);

    const vaultAccountInfo = await provider.connection.getAccountInfo(vault);
    console.log("Vault Account Info (raw):", vaultAccountInfo);

    const vaultBalance = await provider.connection.getBalance(vault);
    console.log("Vault Balance (SOL):", vaultBalance / anchor.web3.LAMPORTS_PER_SOL);
  };

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accountsPartial({
        signer: provider.wallet.publicKey,
        vaultState,
        vault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Tx:", tx);
    await logVaultInfo();
  });

  it("Deposit 2 SOL", async () => {
    const preBalance = await provider.connection.getBalance(vault);
    console.log("Pre-vault Balance (SOL):", preBalance / anchor.web3.LAMPORTS_PER_SOL);

    const tx = await program.methods
      .deposit(new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL))
      .accountsPartial({
        vault,
        vaultState,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Tx:", tx);
    await logVaultInfo();
  });

  it("Withdraw 1 SOL", async () => {
    const preBalance = await provider.connection.getBalance(vault);
    console.log("Pre-vault Balance (SOL):", preBalance / anchor.web3.LAMPORTS_PER_SOL);

    const tx = await program.methods
      .withdraw(new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL))
      .accountsPartial({
        vault,
        vaultState,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Tx:", tx);
    await logVaultInfo();
  });

  it("Close Vault", async () => {
    const preBalance = await provider.connection.getBalance(vault);
    console.log("Pre-vault Balance (SOL):", preBalance / anchor.web3.LAMPORTS_PER_SOL);

    const tx = await program.methods
      .close()
      .accountsPartial({
        vault,
        vaultState,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Tx:", tx);
    await logVaultInfo();
  });
});
