export class Path {
    order: number;

    name: string;

    path: string;

    label: string;

    icon: string;

    hasParams: boolean;

    showInNav: boolean;

    constructor(order: number, name: string, path: string, label: string, icon: string = '', showInNav: boolean = false, hasParams: boolean = false) {
        this.order = order;
        this.name = name;
        this.path = path;
        this.label = label;
        this.icon = icon;
        this.hasParams = hasParams;
        this.showInNav = showInNav;
    }

    getPath(...args: string[]): string {
        let { path } = this;
        if (this.hasParams) {
            // Substitute url arguments (.e.g ":id" or ":group") with args
            // eslint-disable-next-line no-unused-vars
            path = path.replace(/:([^/]+)/g, (_match, _p1) => args.shift() || '');
        }
        return path;
    }
}

const paths = {
    login: new Path(0, 'login', '/auth/login', 'Login'),
    logout: new Path(1, 'logout', '/auth/logout', 'Logout'),

    home: new Path(10, 'home', '/home', 'Dashboard', 'home', true),

    run: new Path(21, 'run', '/run', 'Run', 'play', true),

    minion: new Path(30, 'minion', '/minions/:minionId/:subPage', 'Minion', '', false, true),
    minions: new Path(31, 'minions', '/minions', 'Minions', 'server', true),

    job: new Path(40, 'job', '/jobs/:jobId', 'Job', '', false, true),
    jobs: new Path(41, 'jobs', '/jobs', 'Jobs', 'briefcase', true),
    events: new Path(42, 'events', '/events', 'Events', 'list-ul', true),

    keys: new Path(50, 'keys', '/keys', 'Keys', 'lock', true),

    // -----

    user: new Path(60, 'user', '/users/:userId', 'User', '', false, true),
    users: new Path(61, 'users', '/users', 'Users', 'user-circle', true),

    settings_page: new Path(70, 'settings_page', '/settings/:settingsPage', 'Settings', '', false, true),
    settings: new Path(71, 'settings', '/settings', 'Settings', 'cog', true),

    preferences_page: new Path(80, 'preferences_page', '/preferences/:preferencesPage', 'Preferences', '', false, true),
    preferences: new Path(81, 'preferences', '/preferences', 'Preferences', 'wrench', true),

    // -----

    notFound: new Path(100, 'notFound', '/not-found', 'Not Found'),
};

export default paths;
