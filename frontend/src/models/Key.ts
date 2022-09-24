export default class Key {
    static fromObject(item: any): Key {
        return new Key(item.id, item.finger, item.state);
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
