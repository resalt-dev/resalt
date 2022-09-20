export default class User {
    static fromObject(data: any): any {
        return new User(
            data.id,
            data.username,
            data.isLocal,
            data.perms,
            data.lastLogin,
            data.permissionGroups,
        );
    }

    id: string;

    username: string;

    isLocal: boolean;

    perms: any[]; // the array contains both objects and strings

    lastLogin: string | null;

    readonly permissionGroups: { readonly id: string; readonly name: string }[];

    constructor(
        id: string,
        username: string,
        isLocal: boolean,
        perms: any[],
        lastLogin: string | null,
        permissionGroups: { id: string; name: string }[],
    ) {
        this.id = id;
        this.username = username;
        this.isLocal = isLocal;
        this.perms = perms;
        this.lastLogin = lastLogin;
        this.permissionGroups = permissionGroups;
    }
}
