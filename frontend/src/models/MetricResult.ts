import MetricResultData from './MetricResultData';

export default class MetricResult {
    static fromObject(item: any): any {
        return new MetricResult(
            item.title,
            item.chart,
            item.labels,
            item.data.map((data: any) => MetricResultData.fromObject(data)),
        );
    }

    title: string;

    chart: string;

    labels: string[];

    data: MetricResultData[];

    constructor(
        title: string,
        chart: string,
        labels: string[],
        data: MetricResultData[],
    ) {
        this.title = title;
        this.chart = chart;
        this.labels = labels;
        this.data = data;
    }
}
