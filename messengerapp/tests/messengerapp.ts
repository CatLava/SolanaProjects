import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Messengerapp } from '../target/types/messengerapp';

describe('messengerapp', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Messengerapp as Program<Messengerapp>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
