<script>
    export let minion;

    $: grains = JSON.parse(minion.grains ?? "{}");

    function formatAsSize(megabytes) {
        if (megabytes < 1024) {
            return `${megabytes} MB`;
        } else {
            return `${(megabytes / 1024).toFixed(2)} GB`;
        }
    }
</script>

<div class="row p-3">
    <div class="col-3">
        <div class="card bg-light mb-3">
            <div class="card-header bg-light">
                <span class="fw-bold">Common</span>
            </div>
            <ul class="list-group list-group-flush">
                <li class="list-group-item bg-light">
                    ID <span class="float-end">{minion.id}</span>
                </li>
                <li class="list-group-item bg-light">
                    F.Q.D.N <span class="float-end"
                        >{grains.fqdn ?? "Unknown"}</span
                    >
                </li>
                <li class="list-group-item bg-light">
                    OS <span class="float-end">{grains.os ?? "Unknown"}</span>
                </li>
                <li class="list-group-item bg-light">
                    OS Version <span class="float-end"
                        >{grains.osrelease ?? "Unknown"} ({grains.oscodename ??
                            "Unknown"})</span
                    >
                </li>
                <li class="list-group-item bg-light">
                    Kernel <span class="float-end"
                        >{grains.kernelrelease ?? "Unknown"}</span
                    >
                </li>
            </ul>
        </div>
    </div>

    <div class="col-3">
        <div class="card bg-light mb-3">
            <div class="card-header bg-light">
                <span class="fw-bold">Hardware</span>
            </div>
            <ul class="list-group list-group-flush">
                <li class="list-group-item bg-light">
                    CPU <span class="float-end"
                        >{grains.cpu_model ?? "Unknown"}</span
                    >
                </li>
                <li class="list-group-item bg-light">
                    Number of CPUs <span class="float-end"
                        >{grains.num_cpus ?? "Unknown"}</span
                    >
                </li>
                <li class="list-group-item bg-light">
                    Memory <span class="float-end"
                        >{formatAsSize(grains.mem_total ?? "Unknown")}</span
                    >
                </li>
                <li class="list-group-item bg-light">
                    Swap <span class="float-end"
                        >{formatAsSize(grains.swap_total ?? "Unknown")}</span
                    >
                </li>
                <li class="list-group-item bg-light">
                    Virtual: <span class="float-end"
                        >{grains.virtual ?? "Unknown"}</span
                    >
                </li>
            </ul>
        </div>
    </div>

    <div class="col-6">
        <div class="card bg-light mb-3">
            <div class="card-header bg-light">
                <span class="fw-bold">Network</span>
            </div>
            <div class="class-body">
                <table class="table">
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
                                <td>{grains.ip_interfaces[inter].join(", ")}</td
                                >
                                <td>{grains.hwaddr_interfaces[inter]}</td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        </div>
    </div>

    <div class="col-3">
        <div class="card bg-light mb-3">
            <div class="card-header bg-light">
                <span class="fw-bold">Timings</span>
            </div>
            <ul class="list-group list-group-flush">
                <li class="list-group-item bg-light">
                    Last seen <span class="float-end"
                        >{minion.last_seen} UTC</span
                    >
                </li>
                <li class="list-group-item bg-light">
                    Conformity checked <span class="float-end"
                        >{minion.last_updated_conformity} UTC</span
                    >
                </li>
                <li class="list-group-item bg-light">
                    Grains fetched at <span class="float-end"
                        >{minion.last_updated_grains} UTC</span
                    >
                </li>
                <li class="list-group-item bg-light">
                    Pillars fetched at <span class="float-end"
                        >{minion.last_updated_pillars} UTC</span
                    >
                </li>
                <li class="list-group-item bg-light">
                    Packages fetched at <span class="float-end"
                        >{minion.last_updated_pkgs} UTC</span
                    >
                </li>
            </ul>
        </div>
    </div>
</div>
