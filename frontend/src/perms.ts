import type User from './models/User';

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

export const P_USER_ADMIN: string = 'user';
export const P_USER_LIST: string = 'user.list';
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
		permission: P_EVENT_LIST,
		title: '[Event] List Events',
		description: 'Allow user to list events.',
	},
	{
		permission: P_SALTKEY_LIST,
		title: '[SaltKey] List All',
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
		permission: P_USER_ADMIN,
		title: '[User] Admin',
		description: 'Allow user to create, manage, and delete other users.',
		danger: true,
	},
	{
		permission: P_USER_LIST,
		title: '[User] List All',
		description: 'Allow user to list users.',
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

function saltWrappedRegex(regex: string): RegExp {
	return new RegExp(`^${regex}$`.replace(/([a-zA-Z0-9])\*/g, '$1.*'));
}

export function hasPermission(
	user: User,
	target: string,
	fun: string,
	args: string[] = [],
	kwargs: any = {},
): boolean {
	// https://docs.saltproject.io/en/latest/topics/eauth/access_control.html
	// [
	//   {
	//     "@resalt": [
	//       "minion.list"
	//     ]
	//   },
	//   {
	//     "01*": [
	//       "grains.items"
	//     ]
	//   },
	//   "test.ping",
	//   {
	//     "bb": [
	//       "grains.items",
	//       "pkg.list"
	//     ]
	//   },
	//   {
	//     "cc": [
	//       {
	//         "grains.items": {
	//           "args": [
	//             "test"
	//           ],
	//           "kwargs": {
	//             "sanitize": true
	//           }
	//         }
	//       }
	//     ]
	//   },
	//   ".*",
	//   {
	//     ".*": [ //host
	//       "grains.items", //function
	//     ]
	//   },
	// ]
	// or
	// [
	//   "test.version",
	//   {
	// 	"mongo\\*": [
	// 	  "network.*"
	// 	]
	//   },
	//   {
	// 	"log\\*": [
	// 	  "network.*",
	// 	  "pkg.*"
	// 	]
	//   },
	//   {
	// 	"G@os:RedHat": [
	// 	  "kmod.*"
	// 	 ]
	//   }
	// ]

	type func = string;
	type funcSection =
		| func
		| {
				[fun: string]:
					| string[]
					| {
							args: string[];
					  }
					| {
							kwargs: any;
					  }
					| {
							args: string[];
							kwargs: any;
					  };
		  };
	// "funSection" covers:
	// - "fun"
	// - { "fun": [] }
	// - { "fun": { "args": [] } }
	// - { "fun": { "kwargs": {} } }
	// - { "fun": { "args": [], "kwargs": {} } }
	type targetSection =
		| func
		| {
				[host: string]: funcSection[];
		  };

	let permissions: targetSection[] = [];
	if (user && user.perms) {
		permissions = user.perms;
	}

	// Both target and fun are REGEX, e.g "log*" or "pkg.*".

	const evaluateFunction = (
		funSection: funcSection,
		fun: string,
		args: string[] = [],
		kwargs: any = {},
	): boolean => {
		if (typeof funSection === 'string') {
			const regex = saltWrappedRegex(funSection);
			return regex.test(fun);
		}
		const keys = Object.keys(funSection);
		if (keys.length !== 1) {
			return false;
		}
		for (const key of keys) {
			const regex = saltWrappedRegex(key);
			if (regex.test(fun)) {
				const value = funSection[key];
				if (typeof value === 'string') {
					return true;
				}
				if (Array.isArray(value)) {
					if (value.length === 0) {
						if (args.length !== 0) {
							return false;
						}
					}
					// Test each arg in the permission argainst "args"
					let result = true;
					for (let i = 0; i < value.length; i++) {
						const regex = saltWrappedRegex(value[i]);
						if (!regex.test(args[i])) {
							result = false;
							break;
						}
					}
					return result;
				}
				if (typeof value === 'object') {
					if (value['args']) {
						if (value['args'].length === 0) {
							if (args.length !== 0) {
								return false;
							}
						}
						// Test each arg in the permission argainst "args"
						for (let i = 0; i < value['args'].length; i++) {
							const regex = saltWrappedRegex(value['args'][i]);
							if (!regex.test(args[i])) {
								return false;
							}
						}
					}
					if (value['kwargs']) {
						const keys = Object.keys(value['kwargs']);
						if (keys.length === 0) {
							if (Object.keys(kwargs).length !== 0) {
								return false;
							}
						}
						// Test each arg in the permission argainst "kwargs"
						for (const key of keys) {
							const regex = saltWrappedRegex(value['kwargs'][key]);
							if (!regex.test(kwargs[key])) {
								return false;
							}
						}
					}
					return true;
				}
			}
		}
		return false;
	};

	const evaluateTarget = (
		targetSection: targetSection,
		target: string,
		fun: string,
		args: string[] = [],
		kwargs: any = {},
	): boolean => {
		if (typeof targetSection === 'string') {
			const regex = saltWrappedRegex(targetSection);
			return regex.test(fun);
		}
		const keys = Object.keys(targetSection);
		if (keys.length !== 1) {
			return false;
		}
		for (const key of keys) {
			const regex = saltWrappedRegex(key);
			if (regex.test(target)) {
				const funSections = targetSection[key];
				for (const funSection of funSections) {
					if (evaluateFunction(funSection, fun, args, kwargs)) {
						return true;
					}
				}
				return false;
			}
		}
		return false;
	};

	for (const permission of permissions) {
		if (evaluateTarget(permission, target, fun, args, kwargs)) {
			return true;
		}
	}
	return false;
}

/**
 * Check if a user has a Resalt permission.
 * @param user The user to validate against, e.g. `$currentUser`.
 * @param permission The permission to check for, e.g. `P_USER_ADMIN`.
 * @returns True if the user has the permission, false otherwise.
 * @example
 * ```ts
 * if (hasResaltPermission($currentUser, P_USER_ADMIN)) {
 *  // Do something
 * }
 * ```
 */
export function hasResaltPermission(user: User, permission: string): boolean {
	return hasPermission(user, '@resalt', permission);
}

export default {};
