export default class PermissionGroup {
	static fromObject(group: any): PermissionGroup {
		return new PermissionGroup(group.id, group.name, group.perms, group.ldap_sync, group.users);
	}

	id: string;

	name: string;

	perms: any[]; // the array contains both objects and strings

	ldapSync: string | null; // ldap dn

	readonly users: { readonly id: string; readonly username: string }[];

	constructor(
		id: string,
		name: string,
		perms: any[],
		ldapSync: string | null,
		users: { id: string; username: string }[],
	) {
		this.id = id;
		this.name = name;
		this.perms = perms;
		this.ldapSync = ldapSync;
		this.users = users;
	}

	hasResaltPermission(perm: string): boolean {
		// Check if perms include { "@resalt": [...] } block,
		// and if it does, check if it contains the permission
		return this.perms.some((block) => {
			if (typeof block !== 'object') {
				return false;
			}

			return Object.keys(block).some((key) => key === '@resalt' && block[key].includes(perm));
		});
	}
}
