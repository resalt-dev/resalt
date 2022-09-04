export default class PermissionGroup {
    id: string;

    name: string;

    perms: any[]; // the array contains both objects and strings

    ldapSync: string | null; // ldap dn

    readonly users: { readonly id: string, readonly username: string }[];

    constructor(
        id: string,
        name: string,
        perms: any[],
        ldapSync: string | null,
        users: { id: string, username: string }[],
    ) {
        this.id = id;
        this.name = name;
        this.perms = perms;
        this.ldapSync = ldapSync;
        this.users = users;
    }
}
