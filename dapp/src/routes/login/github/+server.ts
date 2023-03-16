import { error, redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET = (({ url }) => {
	const ghRedirectUrl = `https://github.com/login/oauth/authorize?client_id=${process.env.GITHUB_CLIENT_ID}&redirect_uri=http://localhost:5173/login/github/callback`;
	console.log('ghRedirectUrl: ', ghRedirectUrl);
	return Response.redirect(ghRedirectUrl, 302);
}) satisfies RequestHandler;
