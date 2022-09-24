<script lang="ts">
    import { onMount } from 'svelte';

    import { writable, type Writable } from 'svelte/store';
    import {
        Card,
        CardBody,
        CardHeader,
        CardSubtitle,
        CardTitle,
        Col,
        Row,
    } from 'sveltestrap';
    import { getMetricResults, showAlert } from '../../controller';
    import { AlertType } from '../../models/AlertType';
    import { theme } from '../../stores';

    import { Pie, Line, Bar } from 'svelte-chartjs';
    import {
        Chart as ChartJS,
        ArcElement,
        BarElement,
        CategoryScale,
        Legend,
        LinearScale,
        LineElement,
        PointElement,
        Title,
        Tooltip,
    } from 'chart.js';
    import Color from '../../models/Color';
    import type MetricResult from '../../models/MetricResult';
    ChartJS.register(
        ArcElement,
        BarElement,
        CategoryScale,
        Legend,
        LinearScale,
        LineElement,
        PointElement,
        Title,
        Tooltip,
    );

    const metrics: Writable<MetricResult[] | null> = writable(null);

    onMount(() => {
        getMetricResults()
            .then((data) => {
                metrics.set(data);
            })
            .catch((err) => {
                showAlert(
                    AlertType.ERROR,
                    'Failed fetching dashboard metrics',
                    err,
                );
            });
    });

    const backgroundColorConfirmity: string[] = [
        Color.green6,
        Color.yellow6,
        Color.red6,
        Color.gray6,
    ];
    const backgroundColorConfirmityHover: string[] = [
        Color.green5,
        Color.yellow5,
        Color.red5,
        Color.gray5,
    ];
</script>

{#if !$metrics}
    <h1>Loading...</h1>
{:else}
    <h1>Dashboard</h1>

    <Row>
        {#each $metrics as metric}
            <Col xs="12" md="6" lg="4">
                <Card class="mb-3 {$theme.dark ? 'bg-dark' : ''}">
                    <CardHeader>
                        <CardTitle class="mb-0">{metric.title}</CardTitle>
                    </CardHeader>
                    <CardBody>
                        {#if metric.chart === 'pie'}
                            <Pie
                                data={{
                                    labels: metric.labels,
                                    datasets: metric.data.map((d) => ({
                                        //label: d.label,
                                        data: d.data,
                                        backgroundColor:
                                            backgroundColorConfirmity,
                                        hoverBackgroundColor:
                                            backgroundColorConfirmityHover,
                                    })),
                                }}
                                options={{
                                    responsive: true,
                                    maintainAspectRatio: false,
                                }}
                            />
                        {:else}
                            <CardSubtitle
                                >Unknown chart type "{metric.chart}".</CardSubtitle
                            >
                        {/if}
                    </CardBody>
                </Card>
            </Col>
        {/each}
    </Row>
{/if}
