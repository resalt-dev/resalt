export default class Job {
    static fromObject(data: any): any {
        return new Job(data.id, data.timestamp, data.jid, data.user, data.minions);
    }

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
