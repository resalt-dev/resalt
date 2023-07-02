import Filter from './Filter';

class MinionPresetRaw {
	id: string;

	name: string;

	filters: string;

	constructor(id: string, name: string, filters: string) {
		this.id = id;
		this.name = name;
		this.filters = filters;
	}
}

export default class MinionPreset {
	static fromObject(data: unknown): MinionPreset {
		const { id, name, filters } = data as MinionPresetRaw;

		const parsedFilters: Filter[] = [];
		let invalidData = false;
		try {
			const temp1 = JSON.parse(filters);
			if (Array.isArray(temp1)) {
				for (const f of temp1) {
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
