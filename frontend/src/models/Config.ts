export default class Key {
    currentVersion: string;

    latestVersion: string;

    constructor(currentVersion, latestVersion) {
        this.currentVersion = currentVersion;
        this.latestVersion = latestVersion;
    }
}
