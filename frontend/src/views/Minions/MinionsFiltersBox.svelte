<script lang="ts">
	import { afterUpdate, beforeUpdate, onDestroy, onMount } from 'svelte';
	import { Button, Col, Input, Label, Row } from 'sveltestrap';
	import { FilterFieldType } from '../../models/FilterFieldType';
	import { FilterOperand } from '../../models/FilterOperand';
	import { TempusDominus, Namespace, extend } from '@eonasdan/tempus-dominus';
	import customDateFormat from '@eonasdan/tempus-dominus/dist/plugins/customDateFormat';
	import Icon from '../../components/Icon.svelte';
	import type Filter from '../../models/Filter';
	import { theme } from '../../stores';
	import type { Unsubscriber, Writable } from 'svelte/store';

	export let filters: Writable<Filter[]>;
	export let updateData: () => void;

	const pickers: TempusDominus[] = [];
	let unsub: Unsubscriber | null;
	let hasRendered: boolean = false;

	function addFilter(): void {
		filters.update((f) => [
			...f,
			{
				fieldType: FilterFieldType.NONE,
				field: '',
				operand: FilterOperand.CONTAINS,
				value: '',
			},
		]);
	}

	function removeFilterByIndex(index: number): void {
		filters.update((f) => f.filter((_, i) => i !== index));
	}

	function resetFilterByIndex(index: number): void {
		filters.update((f) => {
			f[index] = {
				fieldType: FilterFieldType.NONE,
				field: '',
				operand: FilterOperand.CONTAINS,
				value: '',
			};
			return f;
		});
	}

	function filterFieldTypeChanged(index: number, event: Event) {
		const target = event.target as HTMLInputElement;
		const newFieldType = target.value as FilterFieldType;

		// Set field to empty once fieldType when changed.
		filters.update((f) => {
			f[index].field = newFieldType === FilterFieldType.OBJECT ? 'id' : '';
			return f;
		});
	}

	function filterFieldChanged(index: number, event: Event) {
		const target = event.target as HTMLInputElement;
		const newField = target.value;

		// Check if fieldType OBJECT and field "last_seen", then set operand to FilterOperand.GREATER_THAN_OR_EQUAL
		filters.update((f) => {
			if (f[index].fieldType === FilterFieldType.OBJECT && newField === 'last_seen') {
				f[index].operand = FilterOperand.GREATER_THAN_OR_EQUAL;
			}
			return f;
		});
	}

	function createDateTimePickers() {
		// Use TempusDominus
		// Loop through all $filters and create a datetime picker for each
		// that has a fieldType of FilterFieldType.OBJECT and a field of "last_seen".

		// Loop over all $filters with index
		$filters.forEach((filter, index) => {
			// Check if fieldType OBJECT and field "last_seen"
			if (filter.fieldType === FilterFieldType.OBJECT && filter.field === 'last_seen') {
				// Create a new TempusDominus datetime picker
				const picker = new TempusDominus(
					document.getElementById(`datetimepicker${index}`),
					{
						localization: {
							format: 'yyyy-MM-dd HH:mm:ss',
						},
						display: {
							theme: $theme.dark ? 'dark' : 'light',
							icons: {
								type: 'icons',
								time: 'bx fs-4 bx-time',
								date: 'bx fs-4 bx-calendar',
								up: 'bx fs-4 bx-up-arrow-alt',
								down: 'bx fs-4 bx-down-arrow-alt',

								previous: 'bx fs-4 bx-chevron-left',
								next: 'bx fs-4 bx-chevron-right',

								// Unused
								today: 'bx fs-5 bx-calendar-check',
								clear: 'bx fs-5 bx-trash',
								close: 'bx fs-5 bx-x',
							},
							buttons: {
								today: false,
								close: false,
								clear: false,
							},
						},
					},
				);

				if (filter.value.length > 0) {
					const parsedDate = picker.dates.parseInput(filter.value);
					picker.dates.setValue(parsedDate);
				}

				// Add an event listener to the picker
				// eslint-disable-next-line @typescript-eslint/no-unused-vars
				picker.subscribe(Namespace.events.change, (e) => {
					// Update the filter's value to the picker's date
					filters.update((f) => {
						f[index].value = picker.dates.picked[0]
							.toISOString()
							.replace('T', ' ')
							.split('.')[0];
						return f;
					});
				});

				pickers.push(picker);
			}
		});
	}

	beforeUpdate(() => {
		// Destroy all pickers
		pickers.forEach((picker) => picker.dispose());
		pickers.length = 0;
	});

	afterUpdate(() => {
		createDateTimePickers();
		hasRendered = true;
	});

	onMount(() => {
		// Enable customDateFormat plugin in Tempus Dominus (datepicker)
		extend(customDateFormat, undefined);
		unsub = filters.subscribe(() => {
			if (hasRendered) {
				updateData();
			}
		});
	});

	onDestroy(() => {
		if (unsub) {
			unsub();
		}
	});
</script>

{#each $filters as filter, i}
	<Row>
		<Col xs="12" lg="3" xl="2">
			<div class="form-floating {i + 1 === $filters.length ? 'mb-0' : 'mb-3'}">
				<Input
					type="select"
					name="select"
					bind:value={filter.fieldType}
					on:change={(event) => {
						filterFieldTypeChanged(i, event);
					}}
				>
					<option
						value={FilterFieldType.NONE}
						selected={filter.fieldType === FilterFieldType.NONE}>None</option
					>
					<option
						value={FilterFieldType.OBJECT}
						selected={filter.fieldType === FilterFieldType.OBJECT}>Minion</option
					>
					<option
						value={FilterFieldType.GRAIN}
						selected={filter.fieldType === FilterFieldType.GRAIN}>Grain</option
					>
					<option
						value={FilterFieldType.PACKAGE}
						selected={filter.fieldType === FilterFieldType.PACKAGE}>Package</option
					>
				</Input>
				<Label>Filter Type</Label>
			</div>
		</Col>
		{#if filter.fieldType !== FilterFieldType.NONE}
			<Col xs="12" lg="5" xl="3">
				<div class="form-floating {i + 1 === $filters.length ? 'mb-0' : 'mb-3'}">
					{#if filter.fieldType === FilterFieldType.OBJECT}
						<Input
							type="select"
							name="select"
							bind:value={filter.field}
							on:change={(event) => {
								filterFieldChanged(i, event);
							}}
						>
							<option value="id" selected={filter.field === 'id'}> Minion ID </option>
							<option value="os_type" selected={filter.field === 'os_type'}>
								OS Type
							</option>
							<option value="last_seen" selected={filter.field === 'last_seen'}>
								Last Seen
							</option>
							<option
								value="conformity_success"
								selected={filter.field === 'conformity_success'}
							>
								Conformity Success
							</option>
							<option
								value="conformity_incorrect"
								selected={filter.field === 'conformity_incorrect'}
							>
								Conformity Incorrect
							</option>
							<option
								value="conformity_error"
								selected={filter.field === 'conformity_error'}
							>
								Conformity Error
							</option>
						</Input>
					{:else}
						<Input type="text" bind:value={filter.field} required />
					{/if}
					{#if filter.fieldType === FilterFieldType.PACKAGE}
						<Label>Package</Label>
					{:else if filter.fieldType === FilterFieldType.GRAIN}
						<Label>Grain (JSONPath)</Label>
					{:else}
						<Label>Field</Label>
					{/if}
				</div>
			</Col>
			<Col xs="12" lg="4" xl="2">
				<div class="form-floating {i + 1 === $filters.length ? 'mb-0' : 'mb-3'}">
					<Input type="select" name="select" bind:value={filter.operand}>
						{#if !(filter.fieldType === FilterFieldType.OBJECT && (filter.field === 'last_seen' || filter.field === 'conformity_success' || filter.field === 'conformity_incorrect' || filter.field === 'conformity_error'))}
							<option
								value={FilterOperand.CONTAINS}
								selected={filter.operand === FilterOperand.CONTAINS}
							>
								contains
							</option>
							<option
								value={FilterOperand.NOT_CONTAINS}
								selected={filter.operand === FilterOperand.NOT_CONTAINS}
							>
								does not contain
							</option>
						{/if}
						<option
							value={FilterOperand.EQUALS}
							selected={filter.operand === FilterOperand.EQUALS}
						>
							equals
						</option>
						<option
							value={FilterOperand.NOT_EQUALS}
							selected={filter.operand === FilterOperand.NOT_EQUALS}
						>
							does not equal
						</option>
						{#if !(filter.fieldType === FilterFieldType.OBJECT && (filter.field === 'last_seen' || filter.field === 'conformity_success' || filter.field === 'conformity_incorrect' || filter.field === 'conformity_error'))}
							<option value={FilterOperand.STARTS_WITH}> starts with </option>
							<option value={FilterOperand.ENDS_WITH}> ends with </option>
						{/if}
						<option value={FilterOperand.GREATER_THAN_OR_EQUAL}> &gt;= </option>
						<option value={FilterOperand.LESS_THAN_OR_EQUAL}> &lt;= </option>
					</Input>
					<Label>Operand</Label>
				</div>
			</Col>
			<Col xs="12" lg="8" xl="3">
				{#if filter.fieldType === FilterFieldType.OBJECT && filter.field === 'last_seen'}
					<div
						class="input-group"
						id="datetimepicker{i}"
						data-td-target-input="nearest"
						data-td-target-toggle="nearest"
					>
						<div class="form-floating {i + 1 === $filters.length ? 'mb-0' : 'mb-3'}">
							<input
								id="datetimepicker{i}Input"
								type="text"
								class="form-control"
								data-td-target="#datetimepicker{i}"
							/>
							<Label>Date</Label>
						</div>
						<span
							class="input-group-text btn-secondary {i + 1 === $filters.length
								? 'mb-0'
								: 'mb-3'}"
							data-td-target="#datetimepicker{i}"
							data-td-toggle="datetimepicker"
						>
							<Icon name="calendar" size="1" />
						</span>
					</div>
				{:else}
					<div class="form-floating {i + 1 === $filters.length ? 'mb-0' : 'mb-3'}">
						<Input
							type={filter.fieldType === FilterFieldType.OBJECT &&
							(filter.field === 'conformity_success' ||
								filter.field === 'conformity_incorrect' ||
								filter.field === 'conformity_error')
								? 'number'
								: 'text'}
							bind:value={filter.value}
							required
						/>
						{#if filter.fieldType === FilterFieldType.PACKAGE}
							<Label>Version</Label>
						{:else}
							<Label>Value</Label>
						{/if}
					</div>
				{/if}
			</Col>
		{/if}
		<Col>
			{#if filter.fieldType !== FilterFieldType.NONE}
				<div style="height: calc(2px + 3.5rem);" class="float-start">
					<Icon
						name="reset"
						size="1.5"
						class="mouse-pointer"
						style="transform: translateY(65%);"
						on:click={() => {
							resetFilterByIndex(i);
						}}
					/>
				</div>
			{/if}
			<Button
				size="sm"
				color="secondary"
				style="height: calc(2px + 3.5rem);"
				class="float-end"
				disabled={$filters.length === 1}
				on:click={() => {
					removeFilterByIndex(i);
				}}
			>
				<Icon name="minus" size="1" style="margin-top: -2px;" />
			</Button>
			<Button
				size="sm"
				color="secondary"
				style="height: calc(2px + 3.5rem);"
				class="float-end me-2"
				disabled={$filters.length === 5}
				on:click={() => {
					addFilter();
				}}
			>
				<Icon name="plus" size="1" style="margin-top: -2px;" />
			</Button>
		</Col>
	</Row>
	{#if i + 1 !== $filters.length}
		<hr class="text-light mt-0" />
	{/if}
{/each}
