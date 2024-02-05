type PrefTheme = 'light' | 'dark';
export default class UserPreferences {
	static fromObject(data: unknown): UserPreferences {
		const { theme } = data as UserPreferences;
		return new UserPreferences(theme);
	}

	theme: PrefTheme;

	constructor(theme: PrefTheme | null) {
		this.theme = theme || 'light';
	}
}
