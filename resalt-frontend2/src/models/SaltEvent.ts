export default class SaltEvent {
	static fromObject(item: unknown): SaltEvent {
		const { id, timestamp, tag, data } = item as SaltEvent;
		return new SaltEvent(id, timestamp, tag, data);
	}

	id: string;

	timestamp: string;

	tag: string;

	data: string;

	constructor(id: string, timestamp: string, tag: string, data: string) {
		this.id = id;
		this.timestamp = timestamp;
		this.tag = tag;
		this.data = data;
	}
}
