import { Logger } from 'tslog';
import type { RequestHandler } from './$types';

export const GET = (({ url }) => {
	const redirectUri = `${url.origin}/login/github/callback`;
	const logger = new Logger();
	logger.info(`login.github.get: redirectUri=${redirectUri}, url: ${JSON.stringify(url)}`);
	const ghRedirectUrl = `https://github.com/login/oauth/authorize?client_id=${process.env.GITHUB_CLIENT_ID}&redirect_uri=${redirectUri}`;
	return Response.redirect(ghRedirectUrl, 302);
}) satisfies RequestHandler;
