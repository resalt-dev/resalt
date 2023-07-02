export default class Key {
	static fromObject(item: unknown): Key {
		const { id, finger, state } = item as Key;
		return new Key(id, finger, state);
	}

	id: string;

	finger: string;

	state: string;

	constructor(id: string, finger: string, state: string) {
		this.id = id;
		this.finger = finger;
		this.state = state;
	}
}
