import { error, redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET = (async ({ url }) => {
	console.log('login/github/callback GET');
	const code = String(url.searchParams.get('code'));

	// authorize code
	const resp = await fetch(
		`https://github.com/login/oauth/authorize?code=${code}&client_id=${process.env.GITHUB_CLIENT_ID}&client_secret=${process.env.GITHUB_CLIENT_SECRET}`,
		{
			method: 'GET',
			headers: {
				accept: 'application/json'
			}
		}
	);
	if (!resp.ok) throw Error(`Failed to authorize code: ${resp.status}`);
	const json = await resp.json();
	console.log('json: ', json);
	throw redirect(300, '/');
}) satisfies RequestHandler;
