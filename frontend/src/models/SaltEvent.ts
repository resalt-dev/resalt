export default class SaltEvent {
    static fromObject(item: any): any {
        return new SaltEvent(item.id, item.timestamp, item.tag, item.data);
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
