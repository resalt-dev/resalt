export type fPerm = { [fun: string]: unknown } | string;

export default class PermissionGroup {
	static fromObject(group: unknown): PermissionGroup {
		const { id, name, perms, users } = group as PermissionGroup;
		return new PermissionGroup(id, name, perms, users);
	}

	id: string;

	name: string;

	perms: fPerm[]; // the array contains both objects and strings

	readonly users: { readonly id: string; readonly username: string }[];

	constructor(
		id: string,
		name: string,
		perms: fPerm[],
		users: { id: string; username: string }[],
	) {
		this.id = id;
		this.name = name;
		this.perms = perms;
		this.users = users;
	}

	hasResaltPermission(perm: string): boolean {
		// Check if perms include { "@resalt": [...] } block,
		// and if it does, check if it contains the permission
		return this.perms.some((rawBlock) => {
			if (typeof rawBlock !== 'object') {
				return false;
			}
			const block = rawBlock as { [key: string]: string[] }; // Assuming @resalt permissions are string[]-only!

			return Object.keys(block).some(
				(key) => key === '@resalt' && (block[key] ?? []).includes(perm),
			);
		});
	}
}
