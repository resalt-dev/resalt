export default class Config {
	static fromObject(data: unknown): Config {
		const { authForwardEnabled, currentVersion, latestVersion, latestNews } = data as Config;
		return new Config(authForwardEnabled, currentVersion, latestVersion, latestNews);
	}

	authForwardEnabled: boolean;

	currentVersion: string;

	latestVersion: string;

	latestNews: string[];

	constructor(
		authForwardEnabled: boolean,
		currentVersion: string,
		latestVersion: string,
		latestNews: string[],
	) {
		this.authForwardEnabled = authForwardEnabled;
		this.currentVersion = currentVersion;
		this.latestNews = latestNews;
		this.latestVersion = latestVersion;
	}
}
