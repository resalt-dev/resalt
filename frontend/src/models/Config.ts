export default class Config {
	static fromObject(data: any): Config {
		return new Config(
			data.authForwardEnabled,
			data.currentVersion,
			data.latestVersion,
			data.latestNews,
			data.themeDefaultColor,
			data.themeDefaultDark,
			data.themeEnableSwitching,
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
