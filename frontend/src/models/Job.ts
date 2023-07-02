export default class Job {
	static fromObject(data: unknown): Job {
		const { id, timestamp, jid, user, minions } = data as Job;
		return new Job(id, timestamp, jid, user, minions);
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
