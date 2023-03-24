import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import jwt from 'jsonwebtoken';
const PROJECT_ID = '1';
import { toBigNumber, Metaplex, sol, type CandyMachineV2Item } from '@metaplex-foundation/js';
import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';

type User = {
	id: number;
	status: string;
	transferable: boolean;
	projectId: number;
	mintAddress: string;
	claimerAddress: string;
	name: string;
	image: string;
	attributes: {
		rewards: string;
		points: string;
	};
};

const mintNFT = async (walletAddress: string, userName: string) => {
	const user = await getUser(userName);
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
	const resp = await fetch(`https://api.underdogprotocol.com/v2/projects/t/${PROJECT_ID}/nfts`, {
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

const getUser = async (userName: string): Promise<User | undefined> => {
	const resp = await fetch(`https://api.underdogprotocol.com/v2/projects/t/${PROJECT_ID}/nfts`, {
		method: 'GET',
		headers: {
			Authorization: `Bearer ${process.env.UNDERDOG_API_KEY}`
		}
	});
	if (!resp.ok)
		throw new Error(
			`Failed to get user information: ${resp.status} for ${resp.url}. ${JSON.stringify(
				await resp.json()
			)}`
		);
	const data = await resp.json();
	const results = data.results as User[];
	return results.find((res) => res.name.toLowerCase() === userName.toLowerCase());
};

const generateClaimableLink = async (user: User): Promise<string> => {
	const resp = await fetch(
		`https://api.underdogprotocol.com/v2/projects/t/${PROJECT_ID}/nfts/${user.id}/claim`,
		{
			method: 'GET',
			headers: {
				Authorization: `Bearer ${process.env.UNDERDOG_API_KEY}`
			}
		}
	);
	if (!resp.ok)
		throw new Error(`Failed to generate claimable link: ${resp.status} for ${resp.url}`);
	const data = await resp.json();
	return data.link;
};

export const GET = (async ({ url, cookies }) => {
	// get wallet address from url
	const walletAddress = String(url.searchParams.get('walletAddress'));
	if (!walletAddress) throw redirect(307, `http://localhost:5173/login/github`);

	// get jwt token
	const jwtToken = cookies.get('ghJwt');
	if (!jwtToken) throw redirect(307, `http://localhost:5173/login/github`);
	const decoded = jwt.decode(jwtToken, process.env.JWT_SECRET);
	if (!decoded) throw redirect(307, `http://localhost:5173/login/github`);
	const accessToken = decoded.token;

	// authorize code
	if (!accessToken) throw redirect(300, '/error');

	// get information about the user
	const getUserResponse = await fetch(`https://api.github.com/user`, {
		method: 'GET',
		headers: {
			Authorization: `Bearer ${accessToken}`,
			accept: 'application/json'
		}
	});
	if (!getUserResponse.ok) throw Error(`Failed to get user information: ${getUserResponse.status}`);
	const userData = await getUserResponse.json();

	const userName = userData.login;
	// mint NFT

	try {
		const user = await mintNFT(walletAddress, userName);
		if (user.status === 'pending') {
			const link = await generateClaimableLink(user);
			return new Response(
				JSON.stringify({
					link
				}),
				{
					status: 200
				}
			);
		}
	} catch (err) {
		return new Response(err, {
			status: 500
		});
	}

	return new Response('Success', {
		status: 200
	});
}) satisfies RequestHandler;
