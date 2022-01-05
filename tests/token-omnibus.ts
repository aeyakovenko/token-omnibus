import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TokenOmnibus } from '../target/types/token_omnibus';

describe('token-omnibus', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.TokenOmnibus as Program<TokenOmnibus>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
