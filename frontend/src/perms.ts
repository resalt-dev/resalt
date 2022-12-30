/* eslint-disable max-len */
export const P_ADMIN_SUPERADMIN: string = 'admin.superadmin';
export const P_ADMIN_GROUP: string = 'admin.group';

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
export const P_MINION_PRESETS_LIST: string = 'minion.presets.list';
export const P_MINION_PRESETS_MANAGE: string = 'minion.presets.manage';
export const P_MINION_GRAINEXPLORER: string = 'minion.grainexplorer';

export const P_JOB_LIST: string = 'job.list';
export const P_JOB_INFO: string = 'job.info';

export const P_EVENT_LIST: string = 'event.list';

export const P_SALTKEY_LIST: string = 'saltkey.list';
export const P_SALTKEY_ACCEPT: string = 'saltkey.accept';
export const P_SALTKEY_REJECT: string = 'saltkey.reject';
export const P_SALTKEY_DELETE: string = 'saltkey.delete';

export const P_USER_LIST: string = 'user.list';
export const P_USER_ADMIN: string = 'user.admin';
export const P_USER_EMAIL: string = 'user.email';
export const P_USER_PASSWORD: string = 'user.password';

export const resaltWebPermissions: {
	permission: string;
	title: string;
	description: string;
	danger?: boolean;
	warning?: boolean;
}[] = [
	{
		permission: P_ADMIN_SUPERADMIN,
		title: '[Super Admin]',
		description: 'Grants ALL permissions in Resalt.',
		danger: true,
	},
	{
		permission: P_ADMIN_GROUP,
		title: '[Admin] Manage Groups',
		description: 'Allow to create, edit, and delete Groups in Resalt.',
		danger: true,
	},
	{
		permission: P_RUN_LIVE,
		title: '[Run] Live',
		description: 'Allow user to run live commands on minions they have access to.',
		danger: true,
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
		warning: true,
	},
	{
		permission: P_RUN_APPROVAL_EXECUTE,
		title: '[Run] [Approval] Execute',
		description: 'Allow user to execute approved approvals.',
		danger: true,
	},
	{
		permission: P_RUN_APPROVAL_RECOUNT,
		title: '[Run] [Approval] Recount',
		description: 'Allow user to reset approvals/rejects on a pending approval.',
		warning: true,
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
		warning: true,
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
		warning: true,
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
			'Allow user to see detailed conformity of minions. User can always see if minion is compliant or not.',
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
	{
		permission: P_MINION_PRESETS_LIST,
		title: '[Minion] [Presets] List',
		description: 'Allow user to list minion presets.',
	},
	{
		permission: P_MINION_PRESETS_MANAGE,
		title: '[Minion] [Presets] Manage',
		description: 'Allow user to create, edit, and delete minion presets.',
		warning: true,
	},
	{
		permission: P_MINION_GRAINEXPLORER,
		title: '[Minion] Grain Explorer',
		description: 'Allow user to use the grain explorer.',
	},
	{
		permission: P_JOB_LIST,
		title: '[Job] List Jobs',
		description: 'Allow user to list jobs.',
	},
	{
		permission: P_JOB_INFO,
		title: '[Job] See Job Info',
		description: 'Allow user to see full info about a specific job.',
	},
	{
		permission: P_EVENT_LIST,
		title: '[Event] List Events',
		description: 'Allow user to list events.',
	},
	{
		permission: P_SALTKEY_LIST,
		title: '[SaltKey] List',
		description: 'Allow user to list salt keys.',
	},
	{
		permission: P_SALTKEY_ACCEPT,
		title: '[SaltKey] Accept',
		description: 'Allow user to accept salt keys.',
		danger: true,
	},
	{
		permission: P_SALTKEY_REJECT,
		title: '[SaltKey] Reject',
		description: 'Allow user to reject salt keys.',
		warning: true,
	},
	{
		permission: P_SALTKEY_DELETE,
		title: '[SaltKey] Delete',
		description: 'Allow user to delete salt keys.',
		danger: true,
	},
	{
		permission: P_USER_LIST,
		title: '[User] List',
		description: 'Allow user to list users.',
	},
	{
		permission: P_USER_ADMIN,
		title: '[User] Admin',
		description: 'Allow user to create, manage, and delete other users.',
		danger: true,
	},
	{
		permission: P_USER_EMAIL,
		title: '[User] Email',
		description:
			'Allow user to change their own email. (LDAP users cannot change their email as it is synced via LDAP)',
	},
	{
		permission: P_USER_PASSWORD,
		title: '[User] Password',
		description:
			'Allow user to change their own password. (LDAP users cannot set or log in with local password)',
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
