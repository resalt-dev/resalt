import { v4 as uuidv4 } from 'uuid';

export default class Message {
	id: string;

	type: string;

	title: string;

	message: unknown;

	constructor(type: string, title: string, message: unknown) {
		this.id = uuidv4();
		this.type = type;
		this.title = title;
		this.message = message;
	}
}
