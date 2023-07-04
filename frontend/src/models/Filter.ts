import { FilterFieldType } from './FilterFieldType';
import { FilterOperand } from './FilterOperand';

export default class Filter {
	static newEmpty(): Filter {
		return new Filter(FilterFieldType.NONE, '', FilterOperand.CONTAINS, '');
	}
	static fromObject(data: unknown): Filter {
		const { fieldType, field, operand, value } = data as Filter;
		if (typeof fieldType !== 'string') {
			throw new Error('Invalid filter fieldType, expected string, was ' + typeof fieldType);
		}
		if (typeof field !== 'string') {
			throw new Error('Invalid filter field, expected string, was ' + typeof field);
		}
		if (typeof operand !== 'string') {
			throw new Error('Invalid filter operand, expected string, was ' + typeof operand);
		}
		if (typeof value !== 'string') {
			throw new Error('Invalid filter value, expected string, was ' + typeof value);
		}

		// Check if fieldType is one of the enum values
		if (!Object.values(FilterFieldType).includes(fieldType)) {
			throw new Error('Invalid filter data, invalid fieldType');
		}
		// Check if operand is one of the enum values
		if (!Object.values(FilterOperand).includes(operand)) {
			throw new Error('Invalid filter data, invalid operand');
		}

		return new Filter(fieldType, field, operand, value);
	}

	fieldType: FilterFieldType;

	field: string;

	operand: FilterOperand;

	value: string;

	constructor(fieldType: FilterFieldType, field: string, operand: FilterOperand, value: string) {
		this.fieldType = fieldType;
		this.field = field;
		this.operand = operand;
		this.value = value;
	}

	isValid(): boolean {
		if (this.fieldType === FilterFieldType.NONE) return false;
		if (this.field === '') return false;
		if (this.field === 'last_seen' && this.value === '') return false;
		if (this.field === 'conformity_success' && this.value === '') return false;
		if (this.field === 'conformity_incorrect' && this.value === '') return false;
		if (this.field === 'conformity_error' && this.value === '') return false;

		return true;
	}

	clone(): any {
		return new Filter(this.fieldType, this.field, this.operand, this.value);
	}
}
