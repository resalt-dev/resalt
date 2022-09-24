export default class Config {
    static fromObject(data: any): Config {
        return new Config(
            data.currentVersion,
            data.latestVersion,
            data.themeColor,
        );
    }

    currentVersion: string;

    latestVersion: string;

    themeColor: string;

    constructor(currentVersion: string, latestVersion: string, themeColor: string) {
        this.currentVersion = currentVersion;
        this.latestVersion = latestVersion;
        this.themeColor = themeColor;
    }
}
