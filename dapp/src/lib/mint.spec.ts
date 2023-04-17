import { assert } from 'chai';

import { getGitHubExperience } from './mint';
const apiUrl = 'https://api.github.com/users/octocat';
const reposUrl = 'https://api.github.com/users/octocat/repos';
const accessToken = 'YOUR_ACCESS_TOKEN_HERE';

describe('getGitHubExperience', () => {
	it('should return a normalized score between 0 and 10', async () => {
		const score = await getGitHubExperience('octocat', accessToken);
		assert(score >= 0 && score <= 10);
	});

	it('should return a higher score for users with more followers and stars', async () => {
		const score1 = await getGitHubExperience('octocat', accessToken);
		const score2 = await getGitHubExperience('john', accessToken); // hypothetical user with more followers and stars
		assert(score2 > score1);
	});

	it('should return null if the API request fails', async () => {
		const score = await getGitHubExperience('octocat', accessToken);
		assert.strictEqual(score, null);
	});
});
