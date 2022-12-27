<script lang="ts">
	import { Button, Modal, ModalBody, ModalHeader, Table } from "sveltestrap";
	import RunClientType from "../../models/RunClientType";
	import type RunCommand from "../../models/RunCommand";
	import { theme } from "../../stores";

    export let command: RunCommand;
    export let close: () => void;
    export let execute: () => void;

</script>

<Modal
    isOpen={command !== null}
    toggle={close}
    class={$theme.dark ? 'theme-dark' : ''}
    contentClassName={$theme.dark ? 'bg-secondary text-white' : ''}
>
    <ModalHeader
        toggle={close}
        class="bg-warning text-dark"
    >
        Live-Run Execution
    </ModalHeader>
    <ModalBody>
        The following command is about to be executed:
        <br />
        <br />

        <!-- Summarize what is about to be run -->
        <Table>
            <tbody>
                <tr>
                    <th style="width: 50%">Client Type</th>
                    <td>{RunClientType.getBaseType(command.client)}</td>
                </tr>
                {#if RunClientType.getBaseType(command.client) === 'local'}
                    <tr>
                        <th>Target Type</th>
                        <td>{command.targetType}</td>
                    </tr>
                    <tr>
                        <th>Target</th>
                        <td>{command.target}</td>
                    </tr>
                {/if}
                <tr>
                    <th>Function</th>
                    <td>{command.fun}</td>
                </tr>
                <tr>
                    <th>Arguments</th>
                    <td>{command.arg.toString()}</td>
                </tr>
                <tr>
                    <th>Keyword Arguments</th>
                    <td>{command.kwarg.toString()}</td>
                </tr>       
                <tr>
                    <th>Async</th>
                    <td>{RunClientType.isAsync(command.client)}</td>
                </tr>
                {#if RunClientType.getBaseType(command.client) === 'local'}
                    <tr>
                        <th>Batch</th>
                        <td
                            >{RunClientType.isBatch(command.client)}
                            {#if RunClientType.isBatch(command.client)}({command.batchSize}){/if}</td
                        >
                    </tr>
                {/if}
            </tbody>
        </Table>

        <br />
        Command-line equivalent:<br />
        <br />

        <!-- Generate the command-line equivalent -->
        <code>{command.toCommandLine()}</code>

        <br />
        <br />
        <br />

        <div class="text-center">
            <Button color="warning" on:click={close}
                >Cancel</Button
            >
            <Button color="danger" on:click={execute}>Execute</Button>
        </div>
    </ModalBody>
</Modal>
