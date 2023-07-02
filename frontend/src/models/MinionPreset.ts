import Filter from './Filter';

class MinionPresetRaw {
	id: string;

	name: string;

	filter: string;

	constructor(id: string, name: string, filters: string) {
		this.id = id;
		this.name = name;
		this.filter = filters;
	}
}

export default class MinionPreset {
	static fromObject(data: unknown): MinionPreset {
		const { id, name, filter } = data as MinionPresetRaw;

		const parsedFilters: Filter[] = [];
		let invalidData = false;
		try {
			const filters = JSON.parse(filter);
			if (Array.isArray(filters)) {
				for (const f of filters) {
					parsedFilters.push(Filter.fromObject(f));
				}
			} else {
				console.warn('Invalid filter data, expected array');
				invalidData = true;
			}
		} catch (e) {
			console.warn('Failed to parse filter data:', e);
			invalidData = true;
		}
		return new MinionPreset(id, name, parsedFilters, invalidData);
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
