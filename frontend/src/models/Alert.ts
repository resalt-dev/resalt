export default class Alert {
    type: string;

    title: string;

    message: string;

    constructor(type: string, title: string, message: string) {
        this.type = type;
        this.title = title;
        this.message = message;
    }
}
