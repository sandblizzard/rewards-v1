import type { RequestHandler } from './$types';
import { redirect } from '@sveltejs/kit';
import { getGithubUser, getNFTUser } from '$lib';
import * as tslog from 'tslog';
/**
 * GET endpoint for getting user NFT data from Underdog
 */
export const GET = (async ({ url, cookies }) => {
	const logger = new tslog.Logger();
	// get wallet address from url
	const walletAddress = String(url.searchParams.get('walletAddress'));
	if (!walletAddress) throw redirect(307, `http://localhost:5173/login/github`);

	//
	const jwtToken = cookies.get('ghJwt');
	if (!jwtToken) {
		logger.error("get.mint.github.status: couldn't get jwt token from cookies");
		return new Response('User not logged into github', {
			status: 401
		});
	}
	let userName: string;
	try {
		const user = await getGithubUser(jwtToken);
		userName = user.login;
	} catch (err) {
		logger.error("get.mint.github.status: couldn't get user data from github", err);
		return new Response(JSON.stringify(err), {
			status: 404
		});
	}

	// get user NFT data
	try {
		const user = await getNFTUser(userName, process.env.UNDERDOG_PROJECT_ID);
		return new Response(JSON.stringify(user), {
			status: 200
		});
	} catch (err) {
		logger.error("get.mint.github.status: couldn't get user data from underdog", err);
		return new Response(JSON.stringify(err), {
			status: 404
		});
	}
}) satisfies RequestHandler;
