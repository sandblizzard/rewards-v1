import type { PageServerLoad } from './$types';

export const load = (async (data) => {
	const ghLoginSuccess = data.url.searchParams.get('ghLoginSuccess');
	return { ghLoginSuccess };
}) satisfies PageServerLoad;
