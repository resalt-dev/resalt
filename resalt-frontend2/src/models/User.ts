import type { fPerm } from './PermissionGroup';
import UserPreferences from './UserPreferences';
export default class User {
	static fromObject(data: unknown): User {
		const { id, username, perms, lastLogin, permissionGroups, email } = data as User;
		const { preferences } = data as { preferences: string };
		return new User(id, username, perms, lastLogin, permissionGroups, email, preferences);
	}

	id: string;

	username: string;

	perms: fPerm[]; // the array contains both objects and strings

	lastLogin: string | null;

	email: string | null;

	readonly permissionGroups: { readonly id: string; readonly name: string }[];

	readonly preferences: UserPreferences;

	constructor(
		id: string,
		username: string,
		perms: fPerm[],
		lastLogin: string | null,
		permissionGroups: { id: string; name: string }[],
		email: string | null,
		preferences: string,
	) {
		this.id = id;
		this.username = username;
		this.perms = perms;
		this.lastLogin = lastLogin;
		this.permissionGroups = permissionGroups;
		this.email = email;
		this.preferences = UserPreferences.fromObject(preferences);
	}
}
