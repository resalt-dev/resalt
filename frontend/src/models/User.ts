export default class User {
    static fromObject(data: any): User {
        return new User(
            data.id,
            data.username,
            data.isLocal,
            data.perms,
            data.lastLogin,
            data.permissionGroups,
            data.email,
            data.ldapSync,
        );
    }

    id: string;

    username: string;

    hasPassword: boolean;

    perms: any[]; // the array contains both objects and strings

    lastLogin: string | null;

    email: string | null;

    ldapSync: string | null;

    readonly permissionGroups: { readonly id: string; readonly name: string }[];

    constructor(
        id: string,
        username: string,
        hasPassword: boolean,
        perms: any[],
        lastLogin: string | null,
        permissionGroups: { id: string; name: string }[],
        email: string | null,
        ldapSync: string | null,
    ) {
        this.id = id;
        this.username = username;
        this.hasPassword = hasPassword;
        this.perms = perms;
        this.lastLogin = lastLogin;
        this.permissionGroups = permissionGroups;
        this.email = email;
        this.ldapSync = ldapSync;
    }
}
