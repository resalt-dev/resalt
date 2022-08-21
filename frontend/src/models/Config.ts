export default class Key {
    currentVersion: string;

    latestVersion: string;

    themeColor: string;

    constructor(currentVersion, latestVersion, themeColor) {
        this.currentVersion = currentVersion;
        this.latestVersion = latestVersion;
        this.themeColor = themeColor;
    }
}
