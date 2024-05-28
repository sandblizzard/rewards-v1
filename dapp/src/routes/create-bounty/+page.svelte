<script lang="ts">
	/** @type {import('./$types').PageData} */
	export let data;
	import { walletStore } from '@svelte-on-solana/wallet-adapter-core';
	import { css } from '@emotion/css';
	import * as anchor from '@project-serum/anchor';
	import * as bountySdk from '../../../../sdk-ts/src/index';
	import { WalletMultiButton } from '@svelte-on-solana/wallet-adapter-ui';
	import * as spl from '@solana/spl-token';
	import { workSpace } from '@svelte-on-solana/wallet-adapter-anchor';
	import { PublicKey, TransactionInstruction } from '@solana/web3.js';
	import { onMount } from 'svelte';
	import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
	import type { CreateBountyInput } from '$lib/types';

	let referrer: string;
	let createBountyInput: CreateBountyInput = data.createBountyInput;
	let properlyLoaded: boolean;
	let errorMessage: string = '';
	let success = false;
	let REDIRECT_IN_SECONDS = 5;
	let startedRedirect = 0;
	let redirectIn = REDIRECT_IN_SECONDS;
	let programLogs: string[] = [];

	onMount(() => {
		// check if the page was loaded properly
		properlyLoaded = Object.keys(data.createBountyInput).every(
			(input) => !!createBountyInput[input]
		);
	});

	const convertToken = async (mint: PublicKey, amount: anchor.BN) => {
		let mintDecimals = new anchor.BN((await spl.getMint($workSpace.connection, mint)).decimals);
		return amount.mul(new anchor.BN(10).pow(mintDecimals));
	};

	const createBounty = async () => {
		errorMessage = '';
		const mint = new PublicKey(createBountyInput.mint);
		const domain = createBountyInput.domain;
		const subDomain = createBountyInput.subDomain;
		const id = createBountyInput.id;
		const amount = await convertToken(mint, new anchor.BN(createBountyInput.bountyAmount));
		console.log('createBounty: programId ', $workSpace.program.programId);
		const bountyPDA = bountySdk.pdas.getBountyPDA($workSpace.program, id);

		const escrowPDA = findProgramAddressSync(
			[bountyPDA[0].toBytes()],
			$workSpace.program.programId
		);

		try {
			const ixs: TransactionInstruction[] = [];
			const creatorAccount = await bountySdk.utils.getOrCreateAssociatedTokenAccountIx(
				$workSpace.connection,
				$walletStore.publicKey,
				mint,
				$walletStore.publicKey
			);
			if (creatorAccount.instruction) ixs.push(creatorAccount.instruction);

			const protocolPDA = bountySdk.pdas.getProtocolPDA($workSpace.program);
			const domainPDA = bountySdk.pdas.getDomainPDA(
				$workSpace.program,
				'github',
				domain,
				subDomain,
				'issues'
			);
			const bountyDenominationPDA = bountySdk.pdas.getDenominationPDA($workSpace.program, mint);
			const createBountyIx = await $workSpace.program.methods
				.createBounty(id, amount)
				.accounts({
					creator: $walletStore.publicKey,
					bounty: bountyPDA[0],
					protocol: protocolPDA[0],
					creatorAccount: creatorAccount.address,
					domain: domainPDA[0],
					bountyDenomination: bountyDenominationPDA[0],
					mint,
					escrow: escrowPDA[0]
				})
				.instruction();
			ixs.push(createBountyIx);

			const getLatestBlockhash = await $workSpace.connection.getLatestBlockhash();
			const messageV0 = new anchor.web3.TransactionMessage({
				payerKey: $walletStore.publicKey,
				recentBlockhash: getLatestBlockhash.blockhash,
				instructions: ixs
			}).compileToV0Message();

			const tx = new anchor.web3.VersionedTransaction(messageV0);
			const versionedTx = await $walletStore.signTransaction(tx);

			const res = await $workSpace.connection.sendTransaction(versionedTx, {
				skipPreflight: true
			});
			const confirm = await $workSpace.connection.confirmTransaction(
				{
					signature: res,
					blockhash: getLatestBlockhash.blockhash,
					lastValidBlockHeight: getLatestBlockhash.lastValidBlockHeight
				},
				'confirmed'
			);
			success = true;
			startedRedirect = Math.floor(Date.now() / 1000);
			setTimeout(() => {
				window.location.assign(referrer);
			}, REDIRECT_IN_SECONDS * 1000);

			setInterval(() => {
				redirectIn = startedRedirect + REDIRECT_IN_SECONDS - Math.floor(Date.now() / 1000);
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

<div class="container mx-auto flex flex-col justify-center items-center">
	<div class="title justify-center items-center flex-shrink-0">
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
					<li>Organization: {createBountyInput.domain}</li>
					<li>Team: {createBountyInput.subDomain}</li>
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
			<div class="flex justify-center items-center w-100">
				<p>
					Transaction data does not seem properly loaded. Try to <a href={referrer}>go back</a>
				</p>
			</div>
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
	</div>
</div>

<style>
	:global(body) {
		display: flex;
		padding: 2rem;
		margin: 0;
		background-color: #242424;
		color: rgba(255, 255, 255, 0.87);
		place-items: center;
		min-width: 320px;
		min-height: 100vh;
	}
</style>
