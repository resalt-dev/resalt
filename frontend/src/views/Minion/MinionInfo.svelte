<script>
    export let minion;
</script>

<div class="row p-3">
    <div class="col-4">
        <div class="card bg-light">
            <div class="card-header bg-light">
                <span class="fw-bold">Status</span>
            </div>
            <div class="card-body">
                <h5 class="card-title">Timestamps</h5>
                Last seen: {minion.last_seen}
                <br />
                Last Conformity update: {minion.last_updated_conformity}
                <br />
                Last Grains update: {minion.last_updated_grains}
                <br />
                Last Pillars update: {minion.last_updated_pillars}
                <br />
                Last Packages update: {minion.last_updated_pkgs}
            </div>
        </div>
    </div>
</div>
