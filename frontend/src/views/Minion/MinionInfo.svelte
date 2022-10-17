<script lang="ts">
    import type { Writable } from 'svelte/store';
    import { Card, CardBody, CardHeader, Col, Row, Table } from 'sveltestrap';
    import type Minion from '../../models/Minion';

    export let tabData: Writable<Minion>;

    $: grains = JSON.parse($tabData.grains ?? '{}');
    $: console.log(grains);

    function formatAsSize(megabytes: number | null | undefined) {
        if (megabytes === undefined) {
            return null;
        }
        if (megabytes < 1024) {
            return `${megabytes} MB`;
        } else {
            return `${(megabytes / 1024).toFixed(2)} GB`;
        }
    }
</script>

<Row>
    <Col xs="6" xxl="3">
        <Card class="mb-3">
            <CardHeader>Common</CardHeader>
            <ul class="list-group list-group-flush">
                <li class="list-group-item">
                    <strong>ID</strong>
                    <span class="float-end">{$tabData.id}</span>
                </li>
                <li class="list-group-item">
                    <strong>F.Q.D.N</strong>
                    <span class="float-end">{grains.fqdn ?? 'Unknown'}</span>
                </li>
                <li class="list-group-item">
                    <strong>OS</strong>
                    <span class="float-end">{grains.os ?? 'Unknown'}</span>
                </li>
                <li class="list-group-item ">
                    <strong>OS Version</strong>
                    <span class="float-end"
                        >{grains.osrelease ?? 'Unknown'} ({grains.oscodename ??
                            'Unknown'})</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Kernel</strong>
                    <span class="float-end"
                        >{grains.kernelrelease ?? 'Unknown'}</span
                    >
                </li>
            </ul>
        </Card>
    </Col>

    <Col xs="6" xxl="3">
        <Card class="mb-3">
            <CardHeader>Hardware</CardHeader>
            <ul class="list-group list-group-flush">
                <li class="list-group-item">
                    <strong>CPU</strong>
                    <span class="float-end"
                        >{grains.cpu_model ?? 'Unknown'}</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Number of CPUs</strong>
                    <span class="float-end">{grains.numCpus ?? 'Unknown'}</span>
                </li>
                <li class="list-group-item">
                    <strong>Memory</strong>
                    <span class="float-end"
                        >{formatAsSize(grains.mem_total) ?? 'Unknown'}</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Swap</strong>
                    <span class="float-end"
                        >{formatAsSize(grains.swap_total) ?? 'Unknown'}</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Virtual</strong>
                    <span class="float-end">{grains.virtual ?? 'Unknown'}</span>
                </li>
            </ul>
        </Card>
    </Col>

    <Col xs="6" xxl="3">
        <Card class="mb-3">
            <CardHeader>DNS</CardHeader>
            <ul class="list-group list-group-flush">
                <li class="list-group-item">
                    <strong>IPv4 DNS</strong>
                    {#each (grains.dns ?? {}).ip4_nameservers ?? [] as dns}
                        <span class="float-end">{dns}</span><br />
                    {/each}
                </li>
                <li class="list-group-item">
                    <strong>IPv6 DNS</strong>
                    {#each (grains.dns ?? {}).ip6_nameservers ?? [] as dns}
                        <span class="float-end">{dns}</span><br />
                    {/each}
                </li>
                <li class="list-group-item">
                    <strong>Search Domains</strong>
                    {#each (grains.dns ?? {}).search ?? [] as search}
                        <span class="float-end">{search}</span><br />
                    {/each}
                </li>
            </ul>
        </Card>
    </Col>

    <Col xs="6" xxl="3">
        <Card class="mb-3">
            <CardHeader>Timings</CardHeader>
            <ul class="list-group list-group-flush">
                <li class="list-group-item">
                    <strong>Last seen</strong>
                    <span class="float-end">{$tabData.lastSeen} UTC</span>
                </li>
                <li class="list-group-item">
                    <strong>Conformity check</strong>
                    <span class="float-end"
                        >{$tabData.lastUpdatedConformity != null
                            ? $tabData.lastUpdatedConformity + ' UTC'
                            : 'Never'}</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Grains fetched</strong>
                    <span class="float-end"
                        >{$tabData.lastUpdatedGrains != null
                            ? $tabData.lastUpdatedGrains + ' UTC'
                            : 'Never'}</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Pillars fetched</strong>
                    <span class="float-end"
                        >{$tabData.lastUpdatedPillars != null
                            ? $tabData.lastUpdatedPillars + ' UTC'
                            : 'Never'}</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Packages fetched</strong>
                    <span class="float-end"
                        >{$tabData.lastUpdatedPkgs != null
                            ? $tabData.lastUpdatedPkgs + ' UTC'
                            : 'Never'}</span
                    >
                </li>
            </ul>
        </Card>
    </Col>

    <Col xs="12">
        <Card class="mb-3">
            <CardHeader>Network</CardHeader>
            <CardBody class="p-0">
                <Table hover class="mb-0">
                    <thead>
                        <tr>
                            <th>Interface</th>
                            <th>Address</th>
                            <th>MAC</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each Object.keys(grains.ip_interfaces ?? {}) as inter}
                            <tr>
                                <th>{inter}</th>
                                <td>
                                    {#each grains.ip_interfaces[inter] as addr}
                                        {addr}<br />
                                    {/each}
                                </td>
                                <td>{grains.hwaddr_interfaces[inter]}</td>
                            </tr>
                        {/each}
                    </tbody>
                </Table>
            </CardBody>
        </Card>
    </Col>
</Row>
