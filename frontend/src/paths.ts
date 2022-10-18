export class Path {
    name: string;

    path: string;

    label: string;

    icon: string;

    hasParams: boolean;

    showInNav: boolean;

    constructor(name: string, path: string, label: string, icon: string = '', showInNav: boolean = false, hasParams: boolean = false) {
        this.name = name;
        this.path = path;
        this.label = label;
        this.icon = icon;
        this.hasParams = hasParams;
        this.showInNav = showInNav;
    }

    getPath(...args: string[]): string {
        let { path } = this;
        console.log(`a1: ${path}`);
        if (this.hasParams) {
            console.log(`a2: ${path}`);
            // Substitute url arguments (.e.g ":id" or ":group") with args
            // eslint-disable-next-line no-unused-vars
            path = path.replace(/:([^/]+)/g, (_match, _p1) => args.shift() || '');
        }
        console.log(`a3: ${path}`);
        return path;
    }
}

const authPaths = [
    new Path('login', '/auth/login', 'Login'),
    new Path('logout', '/auth/logout', 'Logout'),
];

const dashboardPaths = [
    new Path('home', '/dashboard/home', 'Dashboard', 'home', true),

    new Path('run', '/dashboard/run', 'Run', 'play', true),

    new Path('minion', '/dashboard/minions/:minionId/:subPage', 'Minion', '', false, true),
    new Path('minions', '/dashboard/minions', 'Minions', 'server', true),

    new Path('job', '/dashboard/jobs/:jobId', 'Job', '', false, true),
    new Path('jobs', '/dashboard/jobs', 'Jobs', 'briefcase', true),
    new Path('events', '/dashboard/events', 'Events', 'list-ul', true),

    new Path('keys', '/dashboard/keys', 'Keys', 'lock', true),

    // -----

    new Path('user', '/dashboard/users/:userId', 'User', '', false, true),
    new Path('users', '/dashboard/users', 'Users', 'user-circle', true),
    new Path('settings_page', '/dashboard/settings/:settingsPage', 'Settings', '', false, true),
    new Path('settings', '/dashboard/settings', 'Settings', 'cog', true),
    new Path('preferences_page', '/dashboard/preferences/:preferencesPage', 'Preferences', '', false, true),
    new Path('preferences', '/dashboard/preferences', 'Preferences', 'wrench', true),
];

const paths: any = new Proxy([
    ...authPaths,
    ...dashboardPaths,
], {
    get: (target: Path[], prop: any, receiver) => {
        let result = target.find(
            (path: Path) => path.name === prop || path.path === prop,
        );
        console.log(`_0: ${result} with ${prop.toString()}`);
        result = result || Reflect.get(target, prop, receiver);
        console.log(`_1: ${result} with ${prop.toString()}`);
        if (!result) {
            throw new Error(`Path not found: ${prop}`);
        }
        return result;
    },
});

export default paths;
