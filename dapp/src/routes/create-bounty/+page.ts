/** @type {import('./$types').PageLoad} */
export function load(loadData): {
	referrer: string;
	createBountyInput: {
		domain: string;
		subDomain: string;
		id: string;
		bountyAmount: number;
		token: string;
		mint: string;
	};
} {
	const referrer = loadData.url.searchParams.get('referrer');
	const createBountyInput = {
		domain: loadData.url.searchParams.get('organization') ?? null,
		subDomain: loadData.url.searchParams.get('team') ?? null,
		id: loadData.url.searchParams.get('id') ?? null,
		bountyAmount: parseInt(loadData.url.searchParams.get('bountyAmount')) ?? null,
		token: loadData.url.searchParams.get('token') ?? null,
		mint: loadData.url.searchParams.get('mint') ?? null
	};
	console.log('createBountyInput', createBountyInput);

	return {
		referrer,
		createBountyInput
	};
}
