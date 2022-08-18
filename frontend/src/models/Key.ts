export default class Key {
    id: string;

    finger: string;

    status: string;

    constructor(id, finger, status) {
        this.id = id;
        this.finger = finger;
        this.status = status;
    }
}
