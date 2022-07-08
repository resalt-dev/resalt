export class Alert {
    type: string;
    title: string;
    message: string;
    
    constructor(type: string, title: string, message: string) {
        this.type = type;
        this.title = title;
        this.message = message;
    }
}

/*
{
                "id": Math.floor(Math.random() * 1000000) + "",
                "last_seen": new Date().toISOString().slice(0, 19),
                "grains": null,
                "pillars": null,
                "pkgs": null,
                "last_updated_grains": null,
                "last_updated_pillars": null,
                "last_updated_pkgs": null,
                "conformity": null,
                "conformity_success": 0,
                "conformity_incorrect": 0,
                "conformity_error": 0,
                "last_updated_conformity": null,
            }*/
export class Minion {
    id: string;
    last_seen: string;
    grains: string | null;
    pillars: string | null;
    pkgs: string | null;
    last_updated_grains: string | null;
    last_updated_pillars: string | null;
    last_updated_pkgs: string | null;
    conformity: string | null;
    conformity_success: number;
    conformity_incorrect: number;
    conformity_error: number;
    last_updated_conformity: string | null;

    constructor(
        id: string,
        last_seen: string,
        grains: string | null = null,
        pillars: string | null = null,
        pkgs: string | null = null,
        last_updated_grains: string | null = null,
        last_updated_pillars: string | null = null,
        last_updated_pkgs: string | null = null,
        conformity: string | null = null,
        conformity_success: number = 0,
        conformity_incorrect: number = 0,
        conformity_error: number = 0,
        last_updated_conformity: string | null = null
    ) {
        this.id = id;
        this.last_seen = last_seen;
        this.grains = grains;
        this.pillars = pillars;
        this.pkgs = pkgs;
        this.last_updated_grains = last_updated_grains;
        this.last_updated_pillars = last_updated_pillars;
        this.last_updated_pkgs = last_updated_pkgs;
        this.conformity = conformity;
        this.conformity_success = conformity_success;
        this.conformity_incorrect = conformity_incorrect;
        this.conformity_error = conformity_error;
        this.last_updated_conformity = last_updated_conformity;
    }
}

export class User {
    id: string;
    username: string;

    constructor(id, username) {
        this.id = id;
        this.username = username;
    }
}

export class SaltEvent {
    id: string;
    timestamp: string;
    tag: string;
    data: string;
}

export enum SortOrder {
    Up,
    Down,
    None
}

export namespace SortOrder {
    export function next(order: SortOrder): SortOrder {
        switch (order) {
            case SortOrder.Up:
                return SortOrder.Down;
            case SortOrder.Down:
                return SortOrder.None;
            default:
                return SortOrder.Up;
        }
    }
}

export class Job {
    id: string;
    timestamp: string;
    jid: string;
    user: string;
    minions: string[];

    constructor(id, timestamp, jid, user, minions) {
        this.id = id;
        this.timestamp = timestamp;
        this.jid = jid;
        this.user = user;
        this.minions = minions;
    }
}