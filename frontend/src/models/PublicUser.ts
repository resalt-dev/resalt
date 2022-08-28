export default class PublicUser {
    id: string;

    username: string;

    isLocal: boolean;

    perms: any[] | null; // the array contains both objects and strings

    lastLogin: string | null;

    constructor(
        id: string,
        username: string,
        isLocal: boolean,
        perms: any[] | null,
        lastLogin: string | null,
    ) {
        this.id = id;
        this.username = username;
        this.isLocal = isLocal;
        this.perms = perms;
        this.lastLogin = lastLogin;
    }
}
