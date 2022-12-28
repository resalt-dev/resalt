/* eslint-disable max-len */
export const P_ADMIN_SUPERADMIN: string = 'admin.superadmin';
export const P_ADMIN_GROUP: string = 'admin.group';
export const P_ADMIN_USER: string = 'admin.user';

export const P_USER_PASSWORD: string = 'user.password';

export const P_RUN_LIVE: string = 'run.live';
export const P_RUN_APPROVAL_LIST: string = 'run.approval.list';
export const P_RUN_APPROVAL_SUBMIT: string = 'run.approval.submit';
export const P_RUN_APPROVAL_JUDGE: string = 'run.approval.judge';
export const P_RUN_APPROVAL_EXECUTE: string = 'run.approval.execute';
export const P_RUN_APPROVAL_RECOUNT: string = 'run.approval.recount'; // Can reset all approvals/rejects
export const P_RUN_APPROVAL_CLOSE: string = 'run.approval.close';
export const P_RUN_APPROVAL_DELETE: string = 'run.approval.delete';
export const P_RUN_TEMPLATE_LIST: string = 'run.template.list';
export const P_RUN_TEMPLATE_LOCAL: string = 'run.template.local'; // Can create local templates
export const P_RUN_TEMPLATE_GLOBAL: string = 'run.template.global'; // Can create global templates

export const P_MINION_LIST: string = 'minion.list';
export const P_MINION_CONFORMITY: string = 'minion.conformity';
export const P_MINION_PILLARS: string = 'minion.pillars';
export const P_MINION_PACKAGES: string = 'minion.packages';

export const resaltWebPermissions: {
	permission: string;
	title: string;
	description: string;
}[] = [
	{
		permission: P_ADMIN_SUPERADMIN,
		title: '[Super Admin]',
		description: 'Grants ALL permissions in Resalt.',
	},
	{
		permission: P_ADMIN_GROUP,
		title: '[Admin] Manage Groups',
		description: 'Allow to create, edit, and delete Groups in Resalt.',
	},
	{
		permission: P_ADMIN_USER,
		title: '[Admin] Manage Users',
		description: 'Allow to create, edit, and delete Users in Resalt.',
	},
	{
		permission: P_USER_PASSWORD,
		title: '[User] Manage Password',
		description:
			'Allow user to change their own password in Resalt. (LDAP users cannot set or log in with local password)',
	},
	{
		permission: P_RUN_LIVE,
		title: '[Run] Live',
		description: 'Allow user to run live commands on minions they have access to.',
	},
	{
		permission: P_RUN_APPROVAL_LIST,
		title: '[Run] [Approval] List',
		description: 'Allow user to list commands pending approval requests.',
	},
	{
		permission: P_RUN_APPROVAL_SUBMIT,
		title: '[Run] [Approval] Submit',
		description: 'Allow user to submit commands for approval.',
	},
	{
		permission: P_RUN_APPROVAL_JUDGE,
		title: '[Run] [Approval] Judge',
		description: 'Allow user to approve or deny approvals.',
	},
	{
		permission: P_RUN_APPROVAL_EXECUTE,
		title: '[Run] [Approval] Execute',
		description: 'Allow user to execute approved approvals.',
	},
	{
		permission: P_RUN_APPROVAL_RECOUNT,
		title: '[Run] [Approval] Recount',
		description: 'Allow user to reset approvals/rejects on a pending approval.',
	},
	{
		permission: P_RUN_APPROVAL_CLOSE,
		title: '[Run] [Approval] Close',
		description: 'Allow user to close (and re-open) voting on approval request.',
	},
	{
		permission: P_RUN_APPROVAL_DELETE,
		title: '[Run] [Approval] Delete',
		description: 'Allow user to delete approval requests.',
	},
	{
		permission: P_RUN_TEMPLATE_LIST,
		title: '[Run] [Templates] List',
		description: 'Allow user to list templates.',
	},
	{
		permission: P_RUN_TEMPLATE_LOCAL,
		title: '[Run] [Templates] Create Local',
		description: 'Allow user to create local templates.',
	},
	{
		permission: P_RUN_TEMPLATE_GLOBAL,
		title: '[Run] [Templates] Create Global',
		description: 'Allow user to create global templates.',
	},
	{
		permission: P_MINION_LIST,
		title: '[Minion] List Minions',
		description: 'Allow user to list minions.',
	},
	{
		permission: P_MINION_CONFORMITY,
		title: '[Minion] See Conformity',
		description:
			'Allow user to see detailed conformity of minions. Even if missing, user can still see if minion is compliant or not.',
	},
	{
		permission: P_MINION_PILLARS,
		title: '[Minion] See Pillars',
		description: 'Allow user to see the pillars of minions.',
	},
	{
		permission: P_MINION_PACKAGES,
		title: '[Minion] See Packages',
		description: 'Allow user to see the packages of minions.',
	},
];

export function hasResaltPermission(permissions: any[], permission: string): boolean {
	// Assume there can be multiple @resalt sections, from ugly merge.
	const resaltPermissions = permissions
		.filter((p) => p['@resalt'] !== undefined)
		.map((p) => p['@resalt'])
		.filter((p) => Array.isArray(p))
		// merge array of arrays
		.map((p) => p as any[])
		.reduce((acc, val) => acc.concat(val), []);

	// If the permission we are looking for is admin.group.create,
	// test both:
	// - admin.group.create
	// - admin.group
	// - admin
	//
	// Additionally, always return true if they have admin.superadmin.
	const testPerms = [permission];
	for (let i = 0; i < permission.length; i += 1) {
		if (permission[i] === '.') {
			testPerms.push(permission.slice(0, i));
		}
	}
	testPerms.push('admin.superadmin');

	// console.log('testPerms', testPerms);
	// console.log('resaltPermissions', resaltPermissions);

	for (const userPermission of resaltPermissions) {
		for (const testPerm of testPerms) {
			if (testPerm === userPermission) {
				return true;
			}
		}
	}
	return false;
}

export default {};
