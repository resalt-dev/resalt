export default class Config {
	static fromObject(data: unknown): Config {
		const {
			authForwardEnabled,
			currentVersion,
			latestVersion,
			latestNews,
			themeDefaultColor,
			themeDefaultDark,
			themeEnableSwitching,
		} = data as Config;
		return new Config(
			authForwardEnabled,
			currentVersion,
			latestVersion,
			latestNews,
			themeDefaultColor,
			themeDefaultDark,
			themeEnableSwitching,
		);
	}

	authForwardEnabled: boolean;

	currentVersion: string;

	latestVersion: string;

	latestNews: string[];

	themeDefaultColor: string;

	themeDefaultDark: boolean;

	themeEnableSwitching: boolean;

	constructor(
		authForwardEnabled: boolean,
		currentVersion: string,
		latestVersion: string,
		latestNews: string[],
		themeDefaultColor: string,
		themeDefaultDark: boolean,
		themeEnableSwitching: boolean,
	) {
		this.authForwardEnabled = authForwardEnabled;
		this.currentVersion = currentVersion;
		this.latestNews = latestNews;
		this.latestVersion = latestVersion;
		this.themeDefaultColor = themeDefaultColor;
		this.themeDefaultDark = themeDefaultDark;
		this.themeEnableSwitching = themeEnableSwitching;
	}
}
