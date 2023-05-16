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
		domain: loadData.url.searchParams.get('domain') ?? null,
		subDomain: loadData.url.searchParams.get('subDomain') ?? null,
		id: loadData.url.searchParams.get('id') ?? null,
		bountyAmount: parseInt(loadData.url.searchParams.get('bountyAmount')) ?? null,
		token: loadData.url.searchParams.get('token') ?? null,
		mint: loadData.url.searchParams.get('mint') ?? null
	};

	return {
		referrer,
		createBountyInput
	};
}
