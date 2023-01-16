import { FilterFieldType } from './FilterFieldType';
import { FilterOperand } from './FilterOperand';

export default class Filter {
	static fromObject(data: any): Filter {
		if (typeof data.fieldType !== "string") {
			throw new Error("Invalid filter fieldType, expected string, was " + typeof data.fieldType);
		}
		if (typeof data.field !== "string") {
			throw new Error("Invalid filter field, expected string, was " + typeof data.field);
		}
		if (typeof data.operand !== "string") {
			throw new Error("Invalid filter operand, expected string, was " + typeof data.operand);
		}
		if (typeof data.value !== "string") {
			throw new Error("Invalid filter value, expected string, was " + typeof data.value);
		}

		// Check if fieldType is one of the enum values
		if (!Object.values(FilterFieldType).includes(data.fieldType)) {
			throw new Error("Invalid filter data, invalid fieldType");
		}
		// Check if operand is one of the enum values
		if (!Object.values(FilterOperand).includes(data.operand)) {
			throw new Error("Invalid filter data, invalid operand");
		}
		
		return new Filter(data.fieldType, data.field, data.operand, data.value);
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
}
