export default class MetricResultData {
    label: string;

    data: number[];

    constructor(label: string, data: number[]) {
        this.label = label;
        this.data = data;
    }
}
