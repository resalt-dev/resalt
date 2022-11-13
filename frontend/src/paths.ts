export class Path {
    order: number;

    name: string;

    private path: string;

    label: string;

    icon: string;

    hasParams: boolean;

    showInNav: boolean;

    constructor(order: number, name: string, path: string, label: string, icon: string = '', showInNav: boolean = false) {
        this.order = order;
        this.name = name;
        this.path = path;
        this.label = label;
        this.icon = icon;
        this.showInNav = showInNav;
    }

    getPath(...args: string[]): string {
        let { path } = this;

        // Substitute url arguments (.e.g ":id" or ":group") with args
        // eslint-disable-next-line no-unused-vars
        path = path.replace(/:([^/]+)/g, (_match, _p1) => args.shift() || '');

        // Trim trailing slashes
        return path.replace(/\/+$/, '');
    }
}

const paths = {
    login: new Path(0, 'login', '/auth/login', 'Login'),

    dashboard: new Path(10, 'dashboard', '/dashboard/:subPage', 'Dashboard', 'home', true),

    run: new Path(21, 'run', '/run/:subPage', 'Run', 'play', true),

    minion: new Path(30, 'minion', '/minion/:minionId/:subPage', 'Minion', '', false),
    minions: new Path(31, 'minions', '/minions/:subPage', 'Minions', 'server', true),

    job: new Path(40, 'job', '/job/:jobId', 'Job', '', false),
    jobs: new Path(41, 'jobs', '/jobs', 'Jobs', 'briefcase', true),

    events: new Path(50, 'events', '/events', 'Events', 'list-ul', true),

    keys: new Path(60, 'keys', '/keys', 'Keys', 'lock', true),

    // -----

    user: new Path(100, 'user', '/user/:userId', 'User', '', false),
    users: new Path(101, 'users', '/users', 'Users', 'user-circle', true),

    settings: new Path(110, 'settings', '/settings/:settingsPage', 'Settings', 'cog', true),

    preferences: new Path(120, 'preferences', '/preferences/:preferencesPage', 'Preferences', 'wrench', true),

    // -----

    notFound: new Path(999, 'notFound', '/not-found', 'Not Found'),
};

export default paths;
