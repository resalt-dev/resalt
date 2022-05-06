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
    new Path ("minion", `/dashboard/minions/:minionId/:subPage`, "Minion", "server", false, true),
    new Path ("minions", `/dashboard/minions`, "Minions", "server", true),
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
