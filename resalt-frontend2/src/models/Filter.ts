import { FilterFieldType } from './FilterFieldType';
import { FilterOperand } from './FilterOperand';

export default class Filter {
	static newEmpty(): Filter {
		return new Filter(FilterFieldType.OBJECT, '', FilterOperand.CONTAINS, '');
	}
	static fromObject(data: unknown): Filter {
		const { fieldType, field, operand, value, id } = data as Filter;
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

		return new Filter(fieldType, field, operand, value, id);
	}

	id: string;

	fieldType: FilterFieldType;

	field: string;

	operand: FilterOperand;

	value: string;

	constructor(
		fieldType: FilterFieldType,
		field: string,
		operand: FilterOperand,
		value: string,
		id?: string,
	) {
		this.id = id || Math.random().toString(36).substring(2);
		this.fieldType = fieldType;
		this.field = field;
		this.operand = operand;
		this.value = value;
	}

	isValid(): boolean {
		const isNumOp = (op: FilterOperand) =>
			op === FilterOperand.EQUALS ||
			op === FilterOperand.NOT_EQUALS ||
			op === FilterOperand.GREATER_THAN_OR_EQUAL ||
			op === FilterOperand.LESS_THAN_OR_EQUAL;
		// Check if fieldType is valid enum
		if (!Object.values(FilterFieldType).includes(this.fieldType)) return false;
		// Check if operand is valid enum
		if (!Object.values(FilterOperand).includes(this.operand)) return false;
		if (this.field === '') return false;
		if (this.field === 'last_seen' && this.value === '') return false;
		if (this.field === 'conformity_success' && !isNumOp(this.operand)) return false;
		if (this.field === 'conformity_incorrect' && !isNumOp(this.operand)) return false;
		if (this.field === 'conformity_error' && !isNumOp(this.operand)) return false;
		// if (this.field === 'last_seen' && !DateTime.isValid(new DateTime(this.value))) return false;
		// TODO: Add linting of last_seen (both ISO8601 and "3 months ago" etc)

		return true;
	}

	clone(): Filter {
		return new Filter(this.fieldType, this.field, this.operand, this.value);
	}
}
