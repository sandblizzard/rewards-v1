import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { getGithubUser, getNFTUser, type User } from '$lib';
import * as tslog from 'tslog';

/**
 * mintNFT mints a new NFT for the user using the underdog API
 * @param walletAddress
 * @param userName
 * @returns
 */
const mintNFT = async (walletAddress: string, userName: string) => {
	const projectId = process.env.UNDERDOG_PROJECT_ID;
	const user = await getNFTUser(userName, projectId);
	if (user) return user;

	const payload = JSON.stringify({
		name: userName,
		image: 'https://cdn.fansided.com/wp-content/blogs.dir/229/files/2016/08/Alolan-Sandslash.jpg',
		attributes: {
			rewards: '0',
			points: '0'
		},
		receiverAddress: walletAddress
	});
	const resp = await fetch(`https://api.underdogprotocol.com/v2/projects/t/${projectId}/nfts`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${process.env.UNDERDOG_API_KEY}`
		},
		body: payload
	});
	if (!resp.ok)
		throw new Error(
			`Failed to mint NFT: ${resp.status}, ${JSON.stringify(await resp.json())}, ${resp.url}`
		);
	const data: User = await resp.json();
	return data;
};

/**
 * GET endpoint for minting a new NFT
 */
export const GET = (async ({ url, cookies }) => {
	const logger = new tslog.Logger();
	// get wallet address from url
	const walletAddress = String(url.searchParams.get('walletAddress'));
	if (!walletAddress) throw redirect(307, url.origin);

	// get jwt token
	const jwtToken = cookies.get('ghJwt');
	if (!jwtToken) throw redirect(307, url.origin);
	let userName: string;
	try {
		const userData = await getGithubUser(jwtToken);
		userName = userData.login;
	} catch (err) {
		logger.error("get.mint.github: couldn't get user data from github", err);
		return new Response(err, {
			status: 500
		});
	}

	try {
		await mintNFT(walletAddress, userName);
	} catch (err) {
		logger.error('get.mint.github: failed to mint NFT ', err);
		return new Response(err, {
			status: 500
		});
	}

	return new Response('Success', {
		status: 200
	});
}) satisfies RequestHandler;
