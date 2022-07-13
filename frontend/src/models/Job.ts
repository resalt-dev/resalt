export default class Job {
    id: string;

    timestamp: string;

    jid: string;

    user: string;

    minions: string[];

    constructor(id: string, timestamp: string, jid: string, user: string, minions: string[]) {
        this.id = id;
        this.timestamp = timestamp;
        this.jid = jid;
        this.user = user;
        this.minions = minions;
    }
}
