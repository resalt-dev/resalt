<script lang="ts">
    import { writable } from 'svelte/store';
    import { Button, Col, FormGroup, Input, Label, Row } from 'sveltestrap';
    import Icon from '../../components/Icon.svelte';
    import type Filter from '../../models/Filter';
    import { FilterFieldType } from '../../models/FilterFieldType';
    import { FilterOperand } from '../../models/FilterOperand';

    export let update: (filters: Filter[]) => void;

    const filters = writable<Filter[]>([
        {
            fieldType: FilterFieldType.NONE,
            field: '',
            operand: FilterOperand.CONTAINS,
            value: '',
        },
    ]);

    function localUpdate() {
        update(
            $filters
                .filter((f) => f.fieldType !== FilterFieldType.NONE)
                .filter((f) => f.field !== ''),
        );
    }

    function onBlur() {
        localUpdate();
    }

    function localAddFilter() {
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

    function localRemoveFilterByIndex(index: number) {
        filters.update((f) => f.filter((_, i) => i !== index));
        localUpdate();
    }

    function localResetFilterByIndex(index: number) {
        filters.update((f) => {
            f[index] = {
                fieldType: FilterFieldType.NONE,
                field: '',
                operand: FilterOperand.CONTAINS,
                value: '',
            };
            return f;
        });
        localUpdate();
    }

    function localFilterFieldTypeChanged(index: number, event: Event) {
        const target = event.target as HTMLInputElement;
        const fieldType = target.value as FilterFieldType;

        // Set field to empty once fieldType when changed.
        filters.update((f) => {
            f[index].field = fieldType === FilterFieldType.OBJECT ? 'id' : '';
            return f;
        });
        localUpdate();
    }
</script>

{#each $filters as filter, i}
    <Row>
        <Col xs="12" lg="3" xl="2">
            <div
                class="form-floating {i + 1 === $filters.length
                    ? 'mb-0'
                    : 'mb-3'}"
            >
                <Input
                    type="select"
                    name="select"
                    bind:value={filter.fieldType}
                    on:change={(event) => {
                        localFilterFieldTypeChanged(i, event);
                    }}
                >
                    <option value={FilterFieldType.NONE} />
                    <option value={FilterFieldType.OBJECT}>Minion</option>
                    <option value={FilterFieldType.GRAIN}>Grain</option>
                    <option value={FilterFieldType.PACKAGE}>Package</option>
                </Input>
                <Label>Filter Type</Label>
            </div>
        </Col>
        {#if filter.fieldType !== FilterFieldType.NONE}
            <Col xs="12" lg="5" xl="3">
                <div
                    class="form-floating {i + 1 === $filters.length
                        ? 'mb-0'
                        : 'mb-3'}"
                >
                    {#if filter.fieldType === FilterFieldType.OBJECT}
                        <Input
                            type="select"
                            name="select"
                            bind:value={filter.field}
                        >
                            <option value="id" selected>Minion ID</option>
                            <option value="os_type">OS Type</option>
                            <option value="last_seen">Last Seen</option>
                            <option value="conformity_success">
                                # Conformity Success
                            </option>
                            <option value="conformity_incorrect">
                                # Conformity Incorrect
                            </option>
                            <option value="conformity_error">
                                # Conformity Error
                            </option>
                        </Input>
                    {:else}
                        <Input
                            type="text"
                            bsSize="sm"
                            bind:value={filter.field}
                            on:blur={onBlur}
                            required
                        />
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
                <div
                    class="form-floating {i + 1 === $filters.length
                        ? 'mb-0'
                        : 'mb-3'}"
                >
                    <Input
                        type="select"
                        name="select"
                        bind:value={filter.operand}
                    >
                        <option value={FilterOperand.CONTAINS}>contains</option>
                        <option value={FilterOperand.NOT_CONTAINS}
                            >does not contain</option
                        >
                        <option value={FilterOperand.EQUALS}>equals</option>
                        <option value={FilterOperand.NOT_EQUALS}
                            >does not equal</option
                        >
                        <option value={FilterOperand.STARTS_WITH}
                            >starts with</option
                        >
                        <option value={FilterOperand.ENDS_WITH}
                            >ends with</option
                        >
                        <option value={FilterOperand.GREATER_THAN_OR_EQUAL}
                            >&gt;=</option
                        >
                        <option value={FilterOperand.LESS_THAN_OR_EQUAL}
                            >&lt;=</option
                        >
                    </Input>
                    <Label>Operand</Label>
                </div>
            </Col>
            <Col xs="12" lg="8" xl="3">
                <div
                    class="form-floating {i + 1 === $filters.length
                        ? 'mb-0'
                        : 'mb-3'}"
                >
                    <Input
                        type="text"
                        name="text"
                        bind:value={filter.value}
                        on:blur={onBlur}
                    />
                    {#if filter.fieldType === FilterFieldType.PACKAGE}
                        <Label>Version</Label>
                    {:else}
                        <Label>Value</Label>
                    {/if}
                </div>
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
                            localResetFilterByIndex(i);
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
                    localRemoveFilterByIndex(i);
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
                    localAddFilter();
                }}
            >
                <Icon name="plus" size="1" style="margin-top: -2px;" />
            </Button>
        </Col>
    </Row>
    {#if i + 1 !== $filters.length}
        <hr class="bg-light mt-0" />
    {/if}
{/each}
