import jwt from 'jsonwebtoken';

export type User = {
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

export const getNFTUser = async (
	userName: string,
	projectId: string
): Promise<User | undefined> => {
	const resp = await fetch(`https://api.underdogprotocol.com/v2/projects/t/${projectId}/nfts`, {
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

export const getGithubUser = async (jwtToken: string): Promise<any> => {
	const decoded = jwt.decode(jwtToken, process.env.JWT_SECRET);
	if (!decoded) throw Error(`Failed to parse jwt`);
	const accessToken = decoded.token;

	// authorize code
	if (!accessToken) throw Error(`Failed to parse jwt`);

	// get information about the user
	const getUserResponse = await fetch(`https://api.github.com/user`, {
		method: 'GET',
		headers: {
			Authorization: `Bearer ${accessToken}`,
			accept: 'application/json'
		}
	});
	if (!getUserResponse.ok) throw Error(`Failed to get user information: ${getUserResponse.status}`);
	return await getUserResponse.json();
};
