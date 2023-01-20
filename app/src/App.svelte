<script lang="ts">
  import { walletStore } from '@svelte-on-solana/wallet-adapter-core';
  import { css } from '@emotion/css';
  import * as anchor from '@project-serum/anchor';
  import {
    WalletProvider,
    WalletMultiButton,
  } from '@svelte-on-solana/wallet-adapter-ui';
  import * as spl from '@solana/spl-token';
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
  let errorMessage: string = '';
  let success = false;
  let REDIRECT_IN_SECONDS = 5;
  let startedRedirect = 0;
  let redirectIn = REDIRECT_IN_SECONDS;
  let programLogs: string[] = [];

  onMount(() => {
    queryParams = new URLSearchParams(window.location.search);
    referrer = queryParams.get('referrer');
    createBountyInput = {
      domain: queryParams.get('domain') ?? 'test',
      subDomain: queryParams.get('subDomain') ?? 'test',
      id: queryParams.get('id') ?? 'test',
      bountyAmount: parseInt(queryParams.get('bountyAmount') ?? '0'),
      token: queryParams.get('token') ?? 'test',
    };
    properlyLoaded = Object.keys(createBountyInput).every(
      (input) => !!createBountyInput[input]
    );
  });

  const convertToken = async (mint: PublicKey, amount: anchor.BN) => {
    let mintDecimals = new anchor.BN(
      (await spl.getMint($workSpace.connection, mint)).decimals
    );
    return amount.mul(new anchor.BN(10).pow(mintDecimals));
  };

  const createBounty = async () => {
    errorMessage = '';
    const mint = new PublicKey('9p2YAK7DXmVZvrXMX3K4pi7t3ZscZwnbXTogHoGFywMN');
    const domain = createBountyInput.domain;
    const subDomain = createBountyInput.subDomain;
    const id = createBountyInput.id;
    const amount = await convertToken(
      mint,
      new anchor.BN(createBountyInput.bountyAmount)
    );

    const bountyPDA = findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('BOUNTY_SANDBLIZZARD'),
        anchor.utils.bytes.utf8.encode(domain),
        anchor.utils.bytes.utf8.encode(subDomain),
        anchor.utils.bytes.utf8.encode(id),
      ],
      $workSpace.program.programId
    );

    const escrowPDA = findProgramAddressSync(
      [bountyPDA[0].toBytes()],
      $workSpace.program.programId
    );

    try {
      const ixs: TransactionInstruction[] = [];
      const creatorAccount = await getOrCreateAssociatedTokenAccountIx(
        $workSpace.connection,
        $walletStore.publicKey,
        mint,
        $walletStore.publicKey
      );
      if (creatorAccount.instruction) ixs.push(creatorAccount.instruction);

      const createBountyIx = await $workSpace.program.methods
        .createBounty(domain, subDomain, id, amount)
        .accounts({
          bounty: bountyPDA[0],
          creatorAccount: creatorAccount.address,
          mint,
          escrow: escrowPDA[0],
        })
        .instruction();
      ixs.push(createBountyIx);

      const messageV0 = new anchor.web3.TransactionMessage({
        payerKey: $walletStore.publicKey,
        recentBlockhash: (await $workSpace.connection.getLatestBlockhash())
          .blockhash,
        instructions: ixs,
      }).compileToV0Message();

      const tx = new anchor.web3.VersionedTransaction(messageV0);
      const versionedTx = await $walletStore.signTransaction(tx);
      console.log('versionedTx: ', versionedTx);

      const res = await $workSpace.connection.sendTransaction(versionedTx, {
        skipPreflight: true,
      });
      console.log('res: ', res);
      const confirm = await $workSpace.connection.confirmTransaction(
        res,
        'confirmed'
      );
      console.log('confirm: ', confirm);
      success = true;
      startedRedirect = Math.floor(Date.now() / 1000);
      setTimeout(() => {
        window.location.assign(referrer);
      }, REDIRECT_IN_SECONDS * 1000);

      setInterval(() => {
        redirectIn =
          startedRedirect + REDIRECT_IN_SECONDS - Math.floor(Date.now() / 1000);
      }, 1000);
    } catch (err) {
      console.log('err: ', err);
      const error = err as anchor.web3.SendTransactionError;
      if (error.name === 'WalletSignTransactionError') {
        errorMessage = 'Failed to sign transaction';
      }
      if (error.name === 'Error') {
        errorMessage = 'Failed to complete transaction';
        programLogs = error.logs;
      }
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
      shadow
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
      {#if errorMessage !== ''}
        <p
          class={css`
            color: red;
          `}
        >
          {errorMessage}
        </p>
      {/if}
      {#if success}
        <p
          class={css`
            color: green;
          `}
        >
          {`Bounty created successfully! Redirecting you in ${redirectIn}`}
        </p>
      {/if}
    {:else}
      <div>Please connect your wallet to create bounty</div>
      <WalletMultiButton />
    {/if}
  </div>
{:else}
  <p>
    Transaction data does not seem properly loaded. Try to <a href={referrer}
      >go back</a
    >
  </p>
{/if}
<a href={referrer}> {'<- Go back'}</a>
{#if programLogs.length > 0}
  <div>
    <h2>{`Program logs ${errorMessage}`}</h2>
  </div>
  <ul
    class={css`
      list-style: none;
      text-align: left;
    `}
  >
    {#each programLogs as log}
      <li>{log}</li>
    {/each}
  </ul>
{/if}
