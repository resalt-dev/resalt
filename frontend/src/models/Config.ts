export default class Config {
    static fromObject(data: any): Config {
        return new Config(
            data.currentVersion,
            data.latestVersion,
            data.defaultThemeColor,
            data.enableThemeSwitching,
        );
    }

    currentVersion: string;

    latestVersion: string;

    defaultThemeColor: string;

    enableThemeSwitching: boolean;

    constructor(
        currentVersion: string,
        latestVersion: string,
        defaultThemeColor: string,
        enableThemeSwitching: boolean,
    ) {
        this.currentVersion = currentVersion;
        this.latestVersion = latestVersion;
        this.defaultThemeColor = defaultThemeColor;
        this.enableThemeSwitching = enableThemeSwitching;
    }
}
