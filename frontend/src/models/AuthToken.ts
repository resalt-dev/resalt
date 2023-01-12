export default class AuthToken {
	static fromObject(data: any): AuthToken {
		return new AuthToken(data.userId, data.token, data.expiry);
	}

	userId: string;

	token: string;

	expiry: number;

	constructor(userId: string, token: string, expiry: number) {
		this.userId = userId;
		this.token = token;
		this.expiry = expiry;
	}
}
