<script lang="ts">
	import { WalletMultiButton } from '@svelte-on-solana/wallet-adapter-ui';
	import { redirect } from '@sveltejs/kit';
	import '../app.css';
</script>

<div class="wrapper-app">
	<div class="title">
		<h1 class="text-8xl sky-300 font-pixel">SANDBLIZZARD</h1>
	</div>

	<div class=" container mx-auto flex flex-col m-6 gap-y-1">
		<p class="font-pixel mx-auto text-4xl">Connect your web2 profiles</p>
		<div class="mx-auto flex flex-row gap-x-0">
			<div class="mx-auto flex flex-row">
				<a
					class="z-0  w-20 border-2 border-black p-1 bg-indigo-200 shadow-md text-center flex items-center"
					href={'/login/github'}
				>
					<p class="mx-auto">Github</p>
				</a>
				<div class="relative right-0.5 z-10 border-2 border-none flex items-center ">
					<div class="border-2 h-5 border-black border-l-0 w-4 p-1 bg-indigo-200" />
				</div>
			</div>

			<div class="mx-auto flex flex-row">
				<div class="relative left-4 z-10  border-2 border-none flex items-center ">
					<div
						class="border-2 h-5 border-black border-l-0 w-4 p-1 transparent bg-zinc-500
                    "
					/>
				</div>
				<div
					class="z-0  w-50 border-2 border-black p-1 bg-indigo-200 shadow-md text-center flex items-center"
				>
					<WalletMultiButton />
				</div>
			</div>
		</div>
	</div>

	<div class="container mx-auto flex flex-col m-6 gap-y-1">
		<h3 class="font-pixel mx-auto text-4xl">Your NFT</h3>
	</div>

	<div class="container mx-auto flex flex-col m-6 gap-y-1">
		<h3 class="font-pixel mx-auto text-4xl">Earn bounties</h3>
	</div>
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
</style>
