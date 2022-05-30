import constants from './constants';
class Path {
    name: string;
    path: string;
    label: string;
    icon: string;
    hasParams: boolean;
    showInNav: boolean;

    constructor(name: string, path: string, label: string, icon: string = "", showInNav: boolean = false, hasParams: boolean = false) {
        this.name = name;
        this.path = path;
        this.label = label;
        this.icon = icon;
        this.hasParams = hasParams;
        this.showInNav = showInNav;
    }

    getPath(...args: string[]) {
        let path = this.path;
        if (this.hasParams) {
            // Substitute url arguments (.e.g ":id" or ":group") with args
            path = path.replace(/:([^/]+)/g, (match, p1) => {
                return args.shift() || "";
            });
        }
        return path;
    }
}

const authPaths = [
    new Path("login", `/auth/login`, "Login"),
    new Path("logout", `/auth/logout`, "Logout"),
];

const dashboardPaths = [
    new Path ("home", `/dashboard/home`, "Dashboard", "home", true),

    new Path ("minion", `/dashboard/minions/:minionId/:subPage`, "Minion", "", false, true),
    new Path ("minions", `/dashboard/minions`, "Minions", "server", true),

    new Path ("run", `/dashboard/run`, "Run", "play", true),

    new Path ("job", `/dashboard/jobs/:jobId`, "Job", "", false, true),
    new Path ("jobs", `/dashboard/jobs`, "Jobs", "briefcase", true),

    new Path ("schedule", `/dashboard/schedules/:scheduleId`, "Schedule", "", false, true),
    new Path ("schedules", `/dashboard/schedules`, "Schedules", "calendar", true),

    new Path ("keys", `/dashboard/keys`, "Keys", "lock", true),

    new Path ("events", `/dashboard/events`, "Events", "list-ul", true),

    // -----

    new Path ("users", `/dashboard/users`, "Users", "user-circle", true),
    new Path ("settings", `/dashboard/settings`, "Settings", "cog", true),

    new Path ("preferences", `/dashboard/preferences`, "Preferences", "cog", false, false),
];

const paths: any = new Proxy ([
    ...authPaths,
    ...dashboardPaths,
], {
    get: (target, prop: any, receiver) => {
        let result;

        for (let path of target) {
            if (path.name == prop || path.path == prop) {
                result = path;
                break;
            }
        }

        if (!result) {
            result = Reflect.get(target, prop, receiver);
        }

        return result;
    },
});

export default paths;
