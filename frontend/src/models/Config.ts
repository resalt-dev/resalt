export default class Config {
	static fromObject(data: any): Config {
		return new Config(
			data.authForwardEnabled,
			data.currentVersion,
			data.latestVersion,
			data.latestNews,
			data.defaultThemeColor,
			data.defaultThemeDark,
			data.enableThemeSwitching,
		);
	}

	authForwardEnabled: boolean;

	currentVersion: string;

	latestVersion: string;

	latestNews: string[];

	defaultThemeColor: string;

	defaultThemeDark: boolean;

	enableThemeSwitching: boolean;

	constructor(
		authForwardEnabled: boolean,
		currentVersion: string,
		latestVersion: string,
		latestNews: string[],
		defaultThemeColor: string,
		defaultThemeDark: boolean,
		enableThemeSwitching: boolean,
	) {
		this.authForwardEnabled = authForwardEnabled;
		this.currentVersion = currentVersion;
		this.latestVersion = latestVersion;
		this.latestNews = latestNews;
		this.defaultThemeColor = defaultThemeColor;
		this.defaultThemeDark = defaultThemeDark;
		this.enableThemeSwitching = enableThemeSwitching;
	}
}
