import Filter from './Filter';

export default class MinionPreset {
	static fromObject(data: any): MinionPreset {
		const filters: Filter[] = [];
		let invalidData = false;
		try {
			const parsed = JSON.parse(data.filter);
			if (Array.isArray(parsed)) {
				for (const f of parsed) {
					filters.push(Filter.fromObject(f));
				}
			} else {
				console.warn('Invalid filter data, expected array');
				invalidData = true;
			}
		} catch (e) {
			console.warn('Failed to parse filter data:', e);
			invalidData = true;
		}
		return new MinionPreset(data.id, data.name, filters, invalidData);
	}

	id: string;

	name: string;

	filters: Filter[];

	invalidData: boolean;

	constructor(id: string, name: string, filters: Filter[], invalidData: boolean) {
		this.id = id;
		this.name = name;
		this.filters = filters;
		this.invalidData = invalidData;
	}
}
