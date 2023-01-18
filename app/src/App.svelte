<script lang="ts">
  import { walletStore } from '@svelte-on-solana/wallet-adapter-core';
  import { css } from '@emotion/css';
  import * as anchor from '@project-serum/anchor';
  import {
    WalletProvider,
    WalletMultiButton,
  } from '@svelte-on-solana/wallet-adapter-ui';
  import {
    AnchorConnectionProvider,
    workSpace,
  } from '@svelte-on-solana/wallet-adapter-anchor';
  import {
    clusterApiUrl,
    PublicKey,
    TransactionInstruction,
  } from '@solana/web3.js';
  import idl from '../../target/idl/bounty.json';
  import {
    PhantomWalletAdapter,
    SolflareWalletAdapter,
  } from '@solana/wallet-adapter-wallets';
  import { onMount } from 'svelte';
  import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
  import { getOrCreateAssociatedTokenAccountIx } from './helper';

  const localStorageKey = 'walletAdapter';
  const network = clusterApiUrl('devnet'); // localhost or mainnet

  type CreateBountyInput = {
    domain: string;
    subDomain: string;
    id: string;
    bountyAmount: number;
    token: string;
  };

  let wallets = [new PhantomWalletAdapter(), new SolflareWalletAdapter()];
  let queryParams: URLSearchParams;
  let referrer: string;
  let createBountyInput: CreateBountyInput;
  let properlyLoaded: boolean;

  onMount(() => {
    console.log('window.global.location: ', window.global.location);
    queryParams = new URLSearchParams(window.location.search);
    referrer = queryParams.get('referrer');
    createBountyInput = {
      domain: queryParams.get('domain') ?? 'test',
      subDomain: queryParams.get('subDomain') ?? 'test',
      id: queryParams.get('id') ?? 'test',
      bountyAmount: parseInt(queryParams.get('bountyAmount') ?? '1000000'),
      token: queryParams.get('token') ?? 'test',
    };
    properlyLoaded = Object.keys(createBountyInput).every(
      (input) => !!createBountyInput[input]
    );
  });

  const createBounty = async () => {
    const bountyPDA = findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('BOUNTY_SANDBLIZZARD'),
        anchor.utils.bytes.utf8.encode(createBountyInput.domain),
        anchor.utils.bytes.utf8.encode(createBountyInput.subDomain),
        anchor.utils.bytes.utf8.encode(createBountyInput.id),
      ],
      $workSpace.systemProgram.programId
    );

    const escrowPDA = findProgramAddressSync(
      [bountyPDA[0].toBytes()],
      $workSpace.systemProgram.programId
    );

    // FIXME: find an ok mint
    try {
      const mint = new PublicKey('bonkKjzREa7pVBRD6nFPAKRaHhS7XpDhhgZCZdGNkuU');
      const ixs: TransactionInstruction[] = [];
      const creatorAccount = await getOrCreateAssociatedTokenAccountIx(
        $workSpace.connection,
        $walletStore.publicKey,
        mint,
        $walletStore.publicKey
      );
      if (creatorAccount.instruction) ixs.push(creatorAccount.instruction);

      const createBountyIx = await $workSpace.program.methods
        .createBounty(
          createBountyInput.domain,
          createBountyInput.subDomain,
          createBountyInput.id,
          new anchor.BN(createBountyInput.bountyAmount)
        )
        .accounts({
          bounty: bountyPDA[0],
          creatorAccount: creatorAccount.address,
          mint,
          escrow: escrowPDA[0],
        })
        .instruction();
      ixs.push(createBountyIx);

      const tx = new anchor.web3.Transaction().add(...ixs);
      const res = await $workSpace.provider.sendAndConfirm(tx);
      console.log('res:  ', res);
      window.location.assign(referrer);
    } catch (err) {
      console.log('failed: ', err);
    }
  };
</script>

<WalletProvider {localStorageKey} {wallets} autoConnect />
<AnchorConnectionProvider {network} {idl} />

{#if properlyLoaded}
  <div
    class={css`
      margin: 10px;
      border: 2px solid white;
      padding: 10px;
    `}
  >
    <h3>Create Sandblizzard Bounty</h3>
    <ul
      class={css`
        list-style: none;
        text-align: left;
      `}
    >
      <li>Domain: {createBountyInput.domain}</li>
      <li>Sub domain: {createBountyInput.subDomain}</li>
      <li>Id: {createBountyInput.id}</li>
      <li>
        Bounty: {createBountyInput.bountyAmount} ${createBountyInput.token}
      </li>
    </ul>
    {#if $walletStore?.connected}
      <button on:click={async () => await createBounty()}>Create bounty</button>
    {:else}
      <div>Please connect your wallet to create bounty</div>
      <WalletMultiButton />
    {/if}
  </div>
{:else}
  <p>
    Transaction data does not seem properly loaded. Try to <a
      href={'https://github.com'}>go back</a
    >
  </p>
{/if}
<a href={'https://github.com'}> {'<- Go back'}</a>
