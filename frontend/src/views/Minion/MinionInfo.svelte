<script>
    export let minion;

    $: grains = JSON.parse(minion.grains ?? "{}");
    $: console.log(grains);

    function formatAsSize(megabytes) {
        if (megabytes == undefined) {
            return null;
        }
        if (megabytes < 1024) {
            return `${megabytes} MB`;
        } else {
            return `${(megabytes / 1024).toFixed(2)} GB`;
        }
    }
</script>

<div class="row p-3">
    <div class="col-6 col-xxl-3">
        <div class="card mb-3">
            <div class="card-header">
                <span class="fw-bold">Common</span>
            </div>
            <ul class="list-group list-group-flush">
                <li class="list-group-item">
                    <strong>ID</strong>
                    <span class="float-end">{minion.id}</span>
                </li>
                <li class="list-group-item">
                    <strong>F.Q.D.N</strong>
                    <span class="float-end">{grains.fqdn ?? "Unknown"}</span>
                </li>
                <li class="list-group-item">
                    <strong>OS</strong>
                    <span class="float-end">{grains.os ?? "Unknown"}</span>
                </li>
                <li class="list-group-item">
                    <strong>OS Version</strong>
                    <span class="float-end"
                        >{grains.osrelease ?? "Unknown"} ({grains.oscodename ??
                            "Unknown"})</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Kernel</strong>
                    <span class="float-end"
                        >{grains.kernelrelease ?? "Unknown"}</span
                    >
                </li>
            </ul>
        </div>
    </div>

    <div class="col-6 col-xxl-3">
        <div class="card mb-3">
            <div class="card-header">
                <span class="fw-bold">Hardware</span>
            </div>
            <ul class="list-group list-group-flush">
                <li class="list-group-item">
                    <strong>CPU</strong>
                    <span class="float-end"
                        >{grains.cpu_model ?? "Unknown"}</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Number of CPUs</strong>
                    <span class="float-end">{grains.num_cpus ?? "Unknown"}</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Memory</strong>
                    <span class="float-end"
                        >{formatAsSize(grains.mem_total) ?? "Unknown"}</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Swap</strong>
                    <span class="float-end"
                        >{formatAsSize(grains.swap_total) ?? "Unknown"}</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Virtual</strong>
                    <span class="float-end">{grains.virtual ?? "Unknown"}</span>
                </li>
            </ul>
        </div>
    </div>

    <div class="col-6 col-xxl-3">
        <div class="card mb-3">
            <div class="card-header">
                <span class="fw-bold">DNS</span>
            </div>
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
        </div>
    </div>

    <div class="col-6 col-xxl-3">
        <div class="card mb-3">
            <div class="card-header">
                <span class="fw-bold">Timings</span>
            </div>
            <ul class="list-group list-group-flush">
                <li class="list-group-item">
                    <strong>Last seen</strong>
                    <span class="float-end">{minion.last_seen} UTC</span>
                </li>
                <li class="list-group-item">
                    <strong>Conformity check</strong>
                    <span class="float-end"
                        >{minion.last_updated_conformity != null
                            ? minion.last_updated_conformity + " UTC"
                            : "Never"}</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Grains fetched</strong>
                    <span class="float-end"
                        >{minion.last_updated_grains != null
                            ? minion.last_updated_grains + " UTC"
                            : "Never"}</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Pillars fetched</strong>
                    <span class="float-end"
                        >{minion.last_updated_pillars != null
                            ? minion.last_updated_pillars + " UTC"
                            : "Never"}</span
                    >
                </li>
                <li class="list-group-item">
                    <strong>Packages fetched</strong>
                    <span class="float-end"
                        >{minion.last_updated_pkgs != null
                            ? minion.last_updated_pkgs + " UTC"
                            : "Never"}</span
                    >
                </li>
            </ul>
        </div>
    </div>

    <div class="col-12">
        <div class="card mb-3">
            <div class="card-header">
                <span class="fw-bold">Network</span>
            </div>
            <div class="class-body px-2">
                <table class="table table-hover">
                    <thead>
                        <tr>
                            <th scope="col">Interface</th>
                            <th scope="col">Address</th>
                            <th scope="col">MAC</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each Object.keys(grains.ip_interfaces ?? {}) as inter}
                            <tr>
                                <th scope="row">{inter}</th>
                                <td>
                                    {#each grains.ip_interfaces[inter] as addr}
                                        {addr}<br />
                                    {/each}
                                </td>
                                <td>{grains.hwaddr_interfaces[inter]}</td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</div>
