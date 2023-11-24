export default class Config {
	static fromObject(data: unknown): Config {
		const {
			authForwardEnabled,
			currentVersion,
			latestVersion,
			latestNews,
			themeDefaultColor,
			themeEnableSwitching,
		} = data as Config;
		return new Config(
			authForwardEnabled,
			currentVersion,
			latestVersion,
			latestNews,
			themeDefaultColor,
			themeEnableSwitching,
		);
	}

	authForwardEnabled: boolean;

	currentVersion: string;

	latestVersion: string;

	latestNews: string[];

	themeDefaultColor: string;

	themeEnableSwitching: boolean;

	constructor(
		authForwardEnabled: boolean,
		currentVersion: string,
		latestVersion: string,
		latestNews: string[],
		themeDefaultColor: string,
		themeEnableSwitching: boolean,
	) {
		this.authForwardEnabled = authForwardEnabled;
		this.currentVersion = currentVersion;
		this.latestNews = latestNews;
		this.latestVersion = latestVersion;
		this.themeDefaultColor = themeDefaultColor;
		this.themeEnableSwitching = themeEnableSwitching;
	}
}
