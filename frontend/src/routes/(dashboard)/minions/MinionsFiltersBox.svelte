<script lang="ts">
	import Icon from '$component/Icon.svelte';
	import { filters } from '$lib/stores';
	import Filter from '$model/Filter';
	import { FilterFieldType } from '$model/FilterFieldType';
	import { FilterOperand } from '$model/FilterOperand';
	import { DateTime, Namespace, TempusDominus } from '@eonasdan/tempus-dominus';
	import { onDestroy, onMount } from 'svelte';

	const pickers: TempusDominus[] = [];

	function addFilter(): void {
		filters.update((f) => [...f, Filter.newEmpty()]);
	}

	function removeFilter(index: number): void {
		filters.update((f) => f.filter((_, i) => i !== index));
	}

	function resetFilter(index: number): void {
		filters.update((f) => {
			f[index] = Filter.newEmpty();
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
			if (
				f[index].fieldType === FilterFieldType.OBJECT &&
				(newField === 'last_seen' ||
					newField === 'conformity_success' ||
					newField === 'conformity_incorrect' ||
					newField === 'conformity_error')
			) {
				f[index].operand = FilterOperand.GREATER_THAN_OR_EQUAL;
			}
			if (newField === 'last_seen' && f[index].value.length === 0) {
				f[index].value = new DateTime().toISOString().replace('T', ' ').split('.')[0];
			}
			if (newField === 'last_seen') {
				createDateTimePicker(f[index]);
			} else {
				// Destroy the picker
				pickers[index].dispose();
				pickers.splice(index, 1);
			}
			return f;
		});
	}

	function createDateTimePicker(filter: Filter) {
		// Check if fieldType OBJECT and field "last_seen"
		if (!(filter.fieldType === FilterFieldType.OBJECT && filter.field === 'last_seen')) {
			return;
		}
		const htmlElement = document.getElementById(`datetimepicker${filter.id}`);
		if (htmlElement === null) {
			console.error('htmlElement is null');
			return;
		}
		// Create a new TempusDominus datetime picker
		const picker = new TempusDominus(htmlElement, {
			localization: {
				format: 'yyyy-MM-dd HH:mm:ss',
				hourCycle: 'h23',
			},
			display: {
				buttons: {
					today: true,
					close: false,
					clear: false,
				},
				calendarWeeks: true,
				icons: {
					type: 'icons',
					time: 'bx fs-4 bx-time',
					date: 'bx fs-4 bx-calendar',
					up: 'bx fs-4 bx-up-arrow-alt',
					down: 'bx fs-4 bx-down-arrow-alt',

					previous: 'bx fs-4 bx-chevron-left',
					next: 'bx fs-4 bx-chevron-right',

					//today: 'bx fs-5 bx-calendar-check',
					today: 'bx fs-5 bx-home',

					// Unused
					clear: 'bx fs-5 bx-trash',
					close: 'bx fs-5 bx-x-circle',
				},
			},
		});

		// If we have a value, set the picker's date to it
		if (filter.value.length > 0) {
			let dt = new DateTime(filter.value);
			if (DateTime.isValid(dt)) {
				console.log('dt', dt);
				picker.dates.setValue(dt);
			}
		}

		const filterId = filter.id;

		// Add an event listener to the picker
		// eslint-disable-next-line @typescript-eslint/no-unused-vars
		picker.subscribe(Namespace.events.change, (_e) => {
			let picked = picker.dates.picked[0];
			if (picked === undefined) {
				return;
			}
			console.log('picked', picked.toISOString().replace('T', ' ').split('.')[0]);
			// Update the filter's value to the picker's date
			filters.update((f) => {
				// Find by filterId
				f.forEach((filter) => {
					if (filter.id === filterId) {
						filter.value = picked.toISOString().replace('T', ' ').split('.')[0];
					}
				});
				return f;
			});
		});

		pickers.push(picker);
	}

	function createDateTimePickers() {
		// Use TempusDominus
		// Loop through all $filters and create a datetime picker for each
		// that has a fieldType of FilterFieldType.OBJECT and a field of "last_seen".

		// Loop over all $filters with index
		$filters.forEach((filter, index) => {
			createDateTimePicker(filter);
		});
	}

	onDestroy(() => {
		// Destroy all pickers
		pickers.forEach((picker) => picker.dispose());
		pickers.length = 0;
	});

	onMount(() => {
		createDateTimePickers();
	});
</script>

{#each $filters as filter, i}
	<div class="row" id="filterRow{filter.id}">
		<div class="col-12 col-lg-3 col-xl-2">
			<div class="form-floating {i + 1 === $filters.length ? 'mb-0' : 'mb-3'}">
				<select
					id="filterFieldType{i}"
					class="form-select"
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
				</select>
				<label class="form-label" for="filterFieldType{i}">Filter Type</label>
			</div>
		</div>
		{#if filter.fieldType !== FilterFieldType.NONE}
			<div class="col-12 col-lg-5 col-xl-3">
				<div class="form-floating {i + 1 === $filters.length ? 'mb-0' : 'mb-3'}">
					{#if filter.fieldType === FilterFieldType.OBJECT}
						<select
							id="filterField{i}"
							class="form-select"
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
						</select>
					{:else}
						<input
							id="filterField{i}"
							type="text"
							class="form-control"
							bind:value={filter.field}
							required
						/>
					{/if}
					{#if filter.fieldType === FilterFieldType.PACKAGE}
						<label class="form-label" for="filterField{i}">Package</label>
					{:else if filter.fieldType === FilterFieldType.GRAIN}
						<label class="form-label" for="filterField{i}">Grain (JSONPath)</label>
					{:else}
						<label class="form-label" for="filterField{i}">Field</label>
					{/if}
				</div>
			</div>
			<div class="col-12 col-lg-4 col-xl-2">
				<div class="form-floating {i + 1 === $filters.length ? 'mb-0' : 'mb-3'}">
					<select id="filterOperand{i}" class="form-select" bind:value={filter.operand}>
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
					</select>
					<label class="form-label" for="filterOperand{i}">Operand</label>
				</div>
			</div>
			<div class="col-12 col-lg-8 col-xl-3">
				{#if filter.fieldType === FilterFieldType.OBJECT && filter.field === 'last_seen'}
					<div
						class="input-group"
						id="datetimepicker{filter.id}"
						data-td-target-input="nearest"
						data-td-target-toggle="nearest"
					>
						<div class="form-floating {i + 1 === $filters.length ? 'mb-0' : 'mb-3'}">
							<input
								id="datetimepicker{filter.id}Input"
								type="text"
								class="form-control {!DateTime.isValid(new DateTime(filter.value))
									? 'is-invalid'
									: ''}"
								data-td-target="#datetimepicker{filter.id}"
							/>
							<label class="form-label" for="datetimepicker{filter.id}Input"
								>Date</label
							>
						</div>
						<span
							class="input-group-text btn-secondary {i + 1 === $filters.length
								? 'mb-0'
								: 'mb-3'}"
							data-td-target="#datetimepicker{filter.id}"
							data-td-toggle="datetimepicker"
						>
							<Icon name="calendar" size="1" />
						</span>
					</div>
				{:else}
					<div class="form-floating {i + 1 === $filters.length ? 'mb-0' : 'mb-3'}">
						<input
							id="filterValue{i}"
							type="text"
							class="form-control"
							bind:value={filter.value}
							required
						/>
						{#if filter.fieldType === FilterFieldType.PACKAGE}
							<label class="form-label" for="filterValue{i}">Version</label>
						{:else}
							<label class="form-label" for="filterValue{i}">Value</label>
						{/if}
					</div>
				{/if}
			</div>
		{/if}
		<div class="col">
			{#if filter.fieldType !== FilterFieldType.NONE}
				<div style="height: calc(2px + 3.5rem);" class="float-start">
					<Icon
						name="reset"
						size="1.5"
						class="mouse-pointer"
						style="transform: translateY(65%);"
						on:click={() => {
							resetFilter(i);
						}}
					/>
				</div>
			{/if}
			<button
				type="button"
				class="btn btn-sm btn-secondary float-end"
				style="height: calc(2px + 3.5rem);"
				disabled={$filters.length === 1}
				on:click={() => {
					removeFilter(i);
				}}
			>
				<Icon name="minus" size="1" style="margin-top: -2px;" />
			</button>
			<button
				type="button"
				class="btn btn-sm btn-secondary float-end me-2"
				style="height: calc(2px + 3.5rem);"
				disabled={$filters.length === 5}
				on:click={() => {
					addFilter();
				}}
			>
				<Icon name="plus" size="1" style="margin-top: -2px;" />
			</button>
		</div>
	</div>
	{#if i + 1 !== $filters.length}
		<hr class="text-light mt-0" />
	{/if}
{/each}
