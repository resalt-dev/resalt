export default class Job {
	static fromObject(data: unknown): Job {
		const { id, timestamp, jid, user, eventId } = data as Job;
		return new Job(id, timestamp, jid, user, eventId);
	}

	id: string;

	timestamp: string;

	jid: string;

	user: string;

	eventId: string[];

	constructor(id: string, timestamp: string, jid: string, user: string, eventId: string[]) {
		this.id = id;
		this.timestamp = timestamp;
		this.jid = jid;
		this.user = user;
		this.eventId = eventId;
	}
}
