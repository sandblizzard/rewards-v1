import type { RequestHandler } from './$types';

export const GET = (({ url }) => {
	//if (userInfo && _userData) return new Response(_userData);
	const ghRedirectUrl = `https://github.com/login/oauth/authorize?client_id=${process.env.GITHUB_CLIENT_ID}&redirect_uri=http://localhost:5173/login/github/callback`;
	return Response.redirect(ghRedirectUrl, 302);
}) satisfies RequestHandler;
