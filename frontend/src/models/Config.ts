export default class Key {
    static fromObject(data: any): any {
        return new Key(
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
