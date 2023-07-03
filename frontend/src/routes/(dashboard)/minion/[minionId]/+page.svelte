<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { MessageType } from '../../../../models/MessageType';
	import { getMinionById } from '$lib/api';
	import { toasts } from '$lib/stores';
	import { writable, type Writable } from 'svelte/store';
	import CopyButton from '../../../../components/CopyButton.svelte';
	import Icon from '../../../../components/Icon.svelte';
	import type Minion from '../../../../models/Minion';
	import paths from '$lib/paths';
	import { hasResaltPermission, P_RUN_LIVE } from '$lib/perms';
	import { currentUser, theme } from '$lib/stores';
	import { formatAsSize } from '$lib/utils';

	$: minionId = $page.params.minionId;
	const minion: Writable<Minion | null> = writable(null);

	onMount(() => {
		getMinionById(minionId)
			.then((data) => {
				minion.set(data);
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed fetching minion: ' + minionId, err);
			});
	});

	$: grains = JSON.parse($minion?.grains ?? '{}');
</script>

<svelte:head>
	<title>Minion {$minion?.id}</title>
</svelte:head>

<div class="row">
	<div class="col-6 col-xxl-3">
		<div class="card mb-3">
			<div class="card-header">Common</div>
			<ul class="list-group list-group-flush">
				<li class="list-group-item">
					<strong class="align-middle">ID</strong>
					<span class="float-end">
						<span class="align-middle">
							{$minion?.id}
						</span>{#if hasResaltPermission($currentUser, P_RUN_LIVE)}
							<a href={paths.run.getPath('live?target=' + $minion?.id)}>
								<button
									type="button"
									class="btn btn-{$theme.color} btn-sm ms-2"
									style="margin-bottom: -0.15rem;margin-top: -0.15rem;"
								>
									<Icon
										name="play"
										size="1"
										align="top"
										style="padding-top: 0.15rem;"
									/>
								</button>
							</a>
						{/if}
						<CopyButton name="Minion ID" value={$minion?.id} />
					</span>
				</li>
				<li class="list-group-item">
					<strong>F.Q.D.N</strong>
					<span class="float-end">
						{#if grains.fqdn}
							{grains.fqdn}
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
				<li class="list-group-item">
					<strong>OS</strong>
					<span class="float-end">
						{#if grains.os}
							{grains.os}
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
				<li class="list-group-item">
					<strong>OS Version</strong>
					<span class="float-end">
						{#if grains.osrelease}
							{grains.osrelease}
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
				<li class="list-group-item">
					<strong class="align-middle">Serial Number</strong>
					<span class="float-end">
						{#if grains.serialnumber}
							<span class="align-middle">{grains.serialnumber}</span>
							<CopyButton name="Serial Number" value={grains.serialnumber} />
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
			</ul>
		</div>
	</div>

	<div class="col-6 col-xxl-3">
		<div class="card mb-3">
			<div class="card-header">Hardware</div>
			<ul class="list-group list-group-flush">
				<li class="list-group-item">
					<strong>CPU</strong>
					<span class="float-end">
						{#if grains.cpu_model}
							{grains.cpu_model}
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
				<li class="list-group-item">
					<strong>Number of CPUs</strong>
					<span class="float-end">
						{#if grains.num_cpus}
							{grains.num_cpus}
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
				<li class="list-group-item">
					<strong>Memory</strong>
					<span class="float-end">
						{#if grains.mem_total}
							{formatAsSize(grains.mem_total)}
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
				<li class="list-group-item">
					<strong>Swap</strong>
					<span class="float-end">
						{#if grains.swap_total}
							{formatAsSize(grains.swap_total)}
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
				<li class="list-group-item">
					<strong>Virtual</strong>
					<span class="float-end">
						{#if grains.virtual}
							{grains.virtual}
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
			</ul>
		</div>
	</div>

	<div class="col-6 col-xxl-3">
		<div class="card mb-3">
			<div class="card-header">DNS</div>
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
			<div class="card-header">Timings</div>
			<ul class="list-group list-group-flush">
				<li class="list-group-item">
					<strong>Last seen</strong>
					<span class="float-end">{$minion?.lastSeen} UTC</span>
				</li>
				<li class="list-group-item">
					<strong>Conformity check</strong>
					<span class="float-end">
						{#if $minion?.lastUpdatedConformity}
							{$minion?.lastUpdatedConformity} UTC
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
				<li class="list-group-item">
					<strong>Grains fetched</strong>
					<span class="float-end">
						{#if $minion?.lastUpdatedGrains}
							{$minion?.lastUpdatedGrains} UTC
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
				<li class="list-group-item">
					<strong>Pillars fetched</strong>
					<span class="float-end">
						{#if $minion?.lastUpdatedPillars}
							{$minion?.lastUpdatedPillars} UTC
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
				<li class="list-group-item">
					<strong>Packages fetched</strong>
					<span class="float-end">
						{#if $minion?.lastUpdatedPkgs}
							{$minion?.lastUpdatedPkgs} UTC
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
			</ul>
		</div>
	</div>

	<div class="col-12">
		<div class="card mb-3">
			<div class="card-header">Network</div>
			<div class="card-body p-0">
				<table class="table table-hover mb-0">
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
				</table>
			</div>
		</div>
	</div>
</div>
