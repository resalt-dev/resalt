import { v4 as uuidv4 } from 'uuid';

export default class Alert {
    id: string;

    type: string;

    title: string;

    message: string;

    constructor(type: string, title: string, message: string) {
        this.id = uuidv4();
        this.type = type;
        this.title = title;
        this.message = message;
    }
}
