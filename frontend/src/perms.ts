/* eslint-disable max-len */
export const P_ADMIN_SUPERADMIN: string = 'admin.superadmin';
export const P_ADMIN_GROUP: string = 'admin.group';
export const P_ADMIN_USER: string = 'admin.user';
export const P_USER_PASSWORD: string = 'user.password';
export const P_USER_THEME: string = 'user.theme';

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
            'Allows the user to create, edit, and delete groups in Resalt.',
    },
    {
        permission: P_ADMIN_USER,
        title: '[Admin] Manage Users',
        description:
            'Allows the user to create, edit, and delete users in Resalt.',
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
    //         'Allows local user to edit their email address in Resalt. LDAP users always sync from LDAP.',
    // },
    {
        permission: P_USER_PASSWORD,
        title: '[User] Manage Password',
        description: 'Allows the user to change their password in Resalt.',
    },
    {
        permission: P_USER_THEME,
        title: '[User] Manage Theme',
        description: 'Allows the user to change their theme in Resalt.',
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
