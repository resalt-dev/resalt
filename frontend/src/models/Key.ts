export default class Key {
    id: string;

    finger: string;

    state: string;

    constructor(id, finger, state) {
        this.id = id;
        this.finger = finger;
        this.state = state;
    }
}
