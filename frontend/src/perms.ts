// [(perm, title, description)]
export const resaltPermissions = [
    [
        'admin.superadmin',
        '[Admin] Super Admin',
        'Grants ALL permissions in Resalt.',
    ],
    [
        'admin.group',
        '[Admin] Group - All',
        'Grants all permissions related to permission group management.',
    ],
    [
        'admin.group.create',
        '[Admin] Group - Create',
        'Grants permission to create new permission groups.',
    ],
    [
        'admin.group.delete',
        '[Admin] Group - Delete',
        'Grants permission to delete existing permission groups.',
    ],
    [
        'admin.group.update',
        '[Admin] Group - Update',
        'Grants permission to update existing permission groups.',
    ],
    [
        'admin.group.manageusers',
        '[Admin] Group - Manage Users',
        'Grants permission to add and remove users from existing permission groups.',
    ],
    [
        'admin.user',
        '[Admin] User - All',
        'Grants all permissions related to user management.',
    ],
    [
        'admin.user.create',
        '[Admin] User - Create',
        'Grants permission to create new users.',
    ],
    [
        'admin.user.delete',
        '[Admin] User - Delete',
        'Grants permission to delete existing users.',
    ],
    [
        'admin.user.update',
        '[Admin] User - Update',
        'Grants permission to update existing users.',
    ],
    [
        'admin.user.resetpassword',
        '[Admin] User - Reset Password',
        'Grants permission to reset the password of existing users.',
    ],
    // [
    //     'admin.user.reset2fa',
    //     '[Admin] User - Reset 2FA',
    //     'Grants permission to reset the 2FA of existing users.',
    // ],
];

export default {};
