import type { FilterFieldType } from './FilterFieldType';
import type { FilterOperand } from './FilterOperand';

export default class Filter {
    static fromObject(data: any): Filter {
        return new Filter(
            data.fieldType,
            data.field,
            data.operand,
            data.value,
        );
    }

    fieldType: FilterFieldType;

    field: string;

    operand: FilterOperand;

    value: string;

    constructor(
        fieldType: FilterFieldType,
        field: string,
        operand: FilterOperand,
        value: string,
    ) {
        this.fieldType = fieldType;
        this.field = field;
        this.operand = operand;
        this.value = value;
    }
}
