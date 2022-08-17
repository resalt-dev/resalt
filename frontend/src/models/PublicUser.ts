export default class PublicUser {
    id: string;

    username: string;

    isLocal: boolean;

    constructor(id, username, isLocal) {
        this.id = id;
        this.username = username;
        this.isLocal = isLocal;
    }
}
