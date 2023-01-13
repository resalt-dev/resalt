<script lang="ts">
	import { Link, type NavigateFn } from 'svelte-navigator';
	import type { Writable } from 'svelte/store';
	import { Button, Card, CardBody, CardHeader, Col, Row, Table } from 'sveltestrap';
	import CopyButton from '../../components/CopyButton.svelte';
	import Icon from '../../components/Icon.svelte';
	import type Minion from '../../models/Minion';
	import paths from '../../paths';
	import { hasResaltPermission, P_RUN_LIVE } from '../../perms';
	import { currentUser, theme } from '../../stores';

	export let navigate: NavigateFn;
	export let minion: Writable<Minion>;

	$: grains = JSON.parse($minion.grains ?? '{}');

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
					<strong class="align-middle">ID</strong>
					<span class="float-end">
						<span class="align-middle">
							{$minion.id}
						</span>{#if hasResaltPermission($currentUser, P_RUN_LIVE)}
							<Link to={paths.run.getPath('live?target=' + $minion.id)}>
								<Button
									color={$theme.color}
									size="sm"
									class="ms-2"
									style="margin-bottom: -0.15rem;margin-top: -0.15rem;"
								>
									<Icon
										name="play"
										size="1"
										align="top"
										style="padding-top: 0.15rem;"
									/>
								</Button>
							</Link>
						{/if}
						<CopyButton name="Minion ID" value={$minion.id} />
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
				<li class="list-group-item ">
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
		</Card>
	</Col>

	<Col xs="6" xxl="3">
		<Card class="mb-3">
			<CardHeader>Hardware</CardHeader>
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
					<span class="float-end">{$minion.lastSeen} UTC</span>
				</li>
				<li class="list-group-item">
					<strong>Conformity check</strong>
					<span class="float-end">
						{#if $minion.lastUpdatedConformity != null}
							{$minion.lastUpdatedConformity} UTC
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
				<li class="list-group-item">
					<strong>Grains fetched</strong>
					<span class="float-end">
						{#if $minion.lastUpdatedGrains != null}
							{$minion.lastUpdatedGrains} UTC
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
				<li class="list-group-item">
					<strong>Pillars fetched</strong>
					<span class="float-end">
						{#if $minion.lastUpdatedPillars != null}
							{$minion.lastUpdatedPillars} UTC
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
				</li>
				<li class="list-group-item">
					<strong>Packages fetched</strong>
					<span class="float-end">
						{#if $minion.lastUpdatedPkgs != null}
							{$minion.lastUpdatedPkgs} UTC
						{:else}
							<em>Unknown</em>
						{/if}
					</span>
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
