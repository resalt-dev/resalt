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
export const P_MINION_GRAINEXPLORER: string = 'minion.grainexplorer';
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
        description:
            'Allow to create, edit, and delete Groups in Resalt.',
    },
    {
        permission: P_ADMIN_USER,
        title: '[Admin] Manage Users',
        description:
            'Allow to create, edit, and delete Users in Resalt.',
    },
    // {
    //     permission: 'user.profile',
    //     title: '[User] Manage Profile',
    //     description:
    //         'Allows the user to edit their profile information in Resalt.',
    // },
    // {
    //     permission: 'user.email',
    //     title: '[User] Manage Email',
    //     description:
    //         'Allows local user to edit their email address in Resalt. (LDAP users always sync from LDAP)',
    // },
    {
        permission: P_USER_PASSWORD,
        title: '[User] Manage Password',
        description: 'Allow user to change their own password in Resalt. (LDAP users cannot set or log in with local password)',
    },
];

export function hasResaltPermission(
    permissions: any[],
    permission: string,
): boolean {
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
