import type MetricResultData from './MetricResultData';

export default class MetricResult {
    title: string;

    chart: string;

    labels: string[];

    data: MetricResultData[];

    constructor(title: string, chart: string, labels: string[], data: MetricResultData[]) {
        this.title = title;
        this.chart = chart;
        this.labels = labels;
        this.data = data;
    }
}
