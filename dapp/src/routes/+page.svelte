<script lang="ts">
	import { WalletMultiButton } from '@svelte-on-solana/wallet-adapter-ui';
	import { walletStore } from '@svelte-on-solana/wallet-adapter-core';
	import '../app.css';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';

	export let data: PageData;

	let error: Record<string, Record<string, any>> = {};

	let mintingNft = false;
	const mintNFT = async (key: string) => {
		if (!$walletStore.connected) {
			error[key] = { success: false, msg: 'Wallet not connected' };
			return;
		}
		mintingNft = true;
		const uri = `/mint/github?walletAddress=${$walletStore.publicKey.toString()}`;
		const resp = await fetch(uri, {
			method: 'Get'
		});
		if (resp.status == 200) {
			const data = await resp.json();
			await goto(data.link);
			error[key] = { success: true, msg: 'Minted' };
			console.log('Minted');
		} else {
			error[key] = { success: false, msg: 'Not minted' };
			console.log('Error');
		}
		mintingNft = false;
	};
</script>

<div class="wrapper-app flex flex-col items-center">
	<div class="title  justify-center items-center">
		<h1 class="text-8xl sky-300 font-pixel">SANDBLIZZARD</h1>
		<h3 class="text-4xl sky-300 text-left font-pixel">LINKER</h3>
	</div>

	<div class=" container mx-auto flex flex-col m-6 gap-y-10">
		<p class="font-pixel mx-auto text-4xl">Connect your web2 profiles</p>

		<div class="mx-auto flex flex-row gap-x-0">
			{#if $walletStore.connected}
				<div class="rounded-md border-4 p-8 border-green-700 ">
					<div class="rounded-md border-4 p-8 border-green-700 ">
						<div class="rounded-md border-4 p-8 border-green-700 ">
							<WalletMultiButton />
						</div>
					</div>
				</div>
			{:else}
				<div class="rounded-md border-4 p-8 border-green-700 border-animation">
					<div class="rounded-md border-4 p-8 border-green-700 border-animation2">
						<div class="rounded-md border-4 p-8 border-green-700 border-animation3">
							<div
								class="z-0  w-50 border-2 border-black p-1 bg-indigo-200 shadow-md text-center flex items-center"
							>
								<WalletMultiButton />
							</div>
						</div>
					</div>
				</div>
			{/if}
		</div>
		<div class="mx-auto flex flex-col gap-2 items-center">
			<p class="font-pixel mx-auto text-4xl">Connect Github</p>
			{#if data.ghLoginSuccess}
				<div
					class={`z-0 w-40 border-2 border-black p-1 shadow-md text-center flex items-center  bg-green-600`}
				>
					<p class={`mx-auto font-pixel ${data.ghLoginSuccess ? 'text-white' : 'text-slate-800'}`}>
						Logged in with Github
					</p>
				</div>
			{:else}
				<a
					class={`z-0 w-40 border-2 border-black p-1 shadow-md text-center flex items-center ${
						data.ghLoginSuccess ? 'bg-green-600 ' : ' bg-indigo-200'
					}`}
					href={'/login/github'}
				>
					<p class={`mx-auto font-pixel ${data.ghLoginSuccess ? 'text-white' : 'text-slate-800'}`}>
						Login with Github
					</p>
				</a>
			{/if}
		</div>
	</div>

	{#if $walletStore.publicKey && data.ghLoginSuccess}
		<div class="container mx-auto flex flex-col m-6 gap-2">
			<h3 class="font-pixel mx-auto text-4xl">Link your accounts</h3>
			<div class="mx-auto flex flex-col gap-x-0">
				<button
					type="button"
					class="w-32 bg-indigo-500 flex flex-row justify-center"
					on:click={() => mintNFT('mint')}
					disabled={mintingNft}
				>
					{#if mintingNft}
						<svg class="animate-spin h-5 w-5 mr-3 " viewBox="0 0 24 24">
							<circle
								class="opacity-25"
								cx="12"
								cy="12"
								r="10"
								stroke="currentColor"
								stroke-width="4"
							/>
							<path
								class="opacity-75"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							/>
						</svg>
						Linking
					{:else}Link{/if}</button
				>
			</div>
		</div>
	{/if}
</div>

<style style="postcss">
	:global(body) {
		padding: 100px;
		margin: 0;
		background-color: #71717a;
	}
	.wrapper-app {
		height: 100vh;
		font-family: 'Gill Sans', 'Gill Sans MT', Calibri, 'Trebuchet MS', sans-serif;
	}
	.title {
		text-align: center;
		color: white;
		font-size: 20px;
		margin-bottom: 40px;
	}

	a {
		color: #676796;
	}

	.address {
		position: absolute;
		right: 30px;
		top: 30px;
		border-radius: 5px;
		padding: 10px;
	}

	.wrapper-content {
		border-radius: 5px;
		padding: 50px;
		width: 400px;
		margin: 0 auto;
		text-align: center;
		margin-bottom: 30px;
	}

	button {
		border: none;
		padding: 16px;
		border-radius: 5px;
		font-size: 16px;
		cursor: pointer;
		color: white;
		background-color: #4e44ce;
	}

	.value {
		font-size: 40px;
		padding: 25px;
		color: white;
	}

	.warning {
		color: #ca4b4b;
		text-align: center;
		padding: 40px;
		font-size: 20px;
	}

	@keyframes bounce {
		0% {
			border-color: green;
			animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
		}
		50%,
		100% {
			border-color: gray;
			animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
		}
	}

	.border-animation {
		animation: 3s bounce infinite;
	}
	.border-animation2 {
		animation: 3s bounce 1s infinite;
	}
	.border-animation3 {
		animation: 3s bounce 2s infinite;
	}
</style>
