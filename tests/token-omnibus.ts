import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TokenOmnibus } from '../target/types/token_omnibus';

describe('token-omnibus', () => {
  const provider = anchor.Provider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider();

  const program = anchor.workspace.TokenOmnibus as Program<TokenOmnibus>;

  it('Is initialized!', async () => {
    // Add your test here.
    var data = Array<byte>(32).fill(0);
    const tx = await program.rpc.initialize(data, {
        accounts: {
            user: provider.wallet.publicKey,
        },
    });
    console.log("Your transaction signature", tx);
    const state = await program.state.fetch();
  });
});
