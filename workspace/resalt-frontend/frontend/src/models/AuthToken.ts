export default class AuthToken {
	static fromObject(data: unknown): AuthToken {
		const { userId, token, expiry } = data as AuthToken;
		return new AuthToken(userId, token, expiry);
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
