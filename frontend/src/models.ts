export class Alert {
    type: string;
    message: string;
    
    constructor(type: string, message: string) {
        this.type = type;
        this.message = message;
    }
}

export class ApiResponse {
    status: number;
    ok: boolean;
    data: any;

    constructor(status, data) {
        this.status = status;
        this.ok = status == 200;
        this.data = data;
    }
}

export class User {
    id: string;
    username: string;
    email: string;

    constructor(id, username, email) {
        this.id = id;
        this.username = username;
        this.email = email;
    }
}