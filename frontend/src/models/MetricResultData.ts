export default class MetricResultData {
    static fromObject(data: any) {
        return new MetricResultData(data.label, data.data);
    }

    label: string;

    data: number[];

    constructor(label: string, data: number[]) {
        this.label = label;
        this.data = data;
    }
}
