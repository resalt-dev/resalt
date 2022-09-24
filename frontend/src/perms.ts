export const resaltWebPermissions: {
    permission: string;
    title: string;
    description: string;
}[] = [
    {
        permission: 'admin.superadmin',
        title: '{Admin} Super Admin',
        description: 'Grants ALL permissions in Resalt.',
    },
    {
        permission: 'admin.group',
        title: '{Admin} Group - All',
        description:
            'Grants all permissions related to permission group management.',
    },
    {
        permission: 'admin.group.create',
        title: '{Admin} Group - Create',
        description: 'Grants permission to create new permission groups.',
    },
    {
        permission: 'admin.group.delete',
        title: '{Admin} Group - Delete',
        description: 'Grants permission to delete existing permission groups.',
    },
    {
        permission: 'admin.group.update',
        title: '{Admin} Group - Update',
        description: 'Grants permission to update existing permission groups.',
    },
    {
        permission: 'admin.group.manageusers',
        title: '{Admin} Group - Manage Users',
        description:
            'Grants permission to add and remove users from existing permission groups.',
    },
    {
        permission: 'admin.user',
        title: '{Admin} User - All',
        description: 'Grants all permissions related to user management.',
    },
    {
        permission: 'admin.user.create',
        title: '{Admin} User - Create',
        description: 'Grants permission to create new users.',
    },
    {
        permission: 'admin.user.delete',
        title: '{Admin} User - Delete',
        description: 'Grants permission to delete existing users.',
    },
    {
        permission: 'admin.user.update',
        title: '{Admin} User - Update',
        description: 'Grants permission to update existing users.',
    },
    {
        permission: 'admin.user.resetpassword',
        title: '{Admin} User - Reset Password',
        description:
            'Grants permission to reset the password of existing users.',
    },
    // {
    //     'admin.user.reset2fa',
    //     '{Admin} User - Reset 2FA',
    //     'Grants permission to reset the 2FA of existing users.',
    // },
];

export default {};
