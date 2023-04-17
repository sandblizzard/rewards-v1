/**
 * checkIfUserHasPaidMintPrice checks if the user has paid the mint price
 */

export const checkIfUserHasPaidMint = async () => {
	// check bounty contract for user's mint status
	return false;
};

/**
 * getGitHubExperience uses the github api to calculate a normalized score for a user
 * @param username
 * @param accessToken
 * @returns
 */
export const getGitHubExperience = async (
	username: string,
	accessToken: string
): Promise<number> => {
	const apiUrl = `https://api.github.com/users/${username}`;
	const reposUrl = `https://api.github.com/users/${username}/repos`;
	const headers = { Authorization: `Bearer ${accessToken}` };

	try {
		const userResponse = await fetch(apiUrl, { headers });
		const reposResponse = await fetch(reposUrl, { headers });

		if (!userResponse.ok || !reposResponse.ok) {
			return null;
		}

		const userData = await userResponse.json();
		const reposData = await reposResponse.json();

		const numFollowers = userData.followers;
		const numStars = reposData.reduce((acc, repo) => acc + repo.stargazers_count, 0);
		const numCommits = reposData.reduce((acc, repo) => acc + repo.commits, 0);
		const numPullRequests = reposData.reduce((acc, repo) => acc + repo.pull_requests, 0);

		const rawScore = numFollowers * 0.5 + numStars * 0.2 + numCommits * 0.1 + numPullRequests * 0.2;
		const normalizedScore = Math.min(Math.max((rawScore - 50) / 5, 0), 10);

		return normalizedScore;
	} catch (error) {
		throw new Error(`Failed to get GitHub experience calculation. Cause ${error}`);
	}
};
