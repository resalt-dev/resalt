import type { fPerm } from './PermissionGroup';
export default class User {
	static fromObject(data: unknown): User {
		const { id, username, perms, lastLogin, permissionGroups, email } = data as User;
		return new User(id, username, perms, lastLogin, permissionGroups, email);
	}

	id: string;

	username: string;

	perms: fPerm[]; // the array contains both objects and strings

	lastLogin: string | null;

	email: string | null;

	readonly permissionGroups: { readonly id: string; readonly name: string }[];

	constructor(
		id: string,
		username: string,
		perms: fPerm[],
		lastLogin: string | null,
		permissionGroups: { id: string; name: string }[],
		email: string | null,
	) {
		this.id = id;
		this.username = username;
		this.perms = perms;
		this.lastLogin = lastLogin;
		this.permissionGroups = permissionGroups;
		this.email = email;
	}
}
