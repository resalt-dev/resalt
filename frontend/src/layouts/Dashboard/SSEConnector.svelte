<script lang="ts">
	import { get } from 'svelte/store';
	import { createSSESocket } from '../../api';
	import { auth } from '../../stores';
	import { socket as socketStore } from '../../stores';

	let stream: EventSource | null;

	$: {
		if ($auth) {
			console.log('Connecting to SSE...');
			openEvents();
		} else {
			console.log('Closing SSE...');
			closeEvents();
		}
	}

	async function openEvents(timeout: number = 1000): Promise<EventSource> {
		if (stream) {
			closeEvents();
		}
		if (get(socketStore).connected) {
			socketStore.set({ connected: false, last_ping: null });
		}

		stream = await createSSESocket();

		stream.addEventListener(
			'message',
			(e) => {
				const data = JSON.parse(e.data);
				console.log('data', data);

				const { content } = data;

				switch (data.type) {
					/* case 'update_minion':
                minionsStore.update((minions: Array<Minion>) => {
                    // minions is a Vector of Minions.
                    // If minion exists, replace it. If not, then add it.
                    const index = minions.findIndex(
                        (minion) => minion.id === content.minion.id,
                    );
                    if (index >= 0) {
                        minions[index] = content.minion;
                    } else {
                        minions.push(content.minion);
                    }
                    return minions;
                });
                break; */
					default:
						console.log('Unknown event type', data.type, content);
				}
			},
			false,
		);

		stream.addEventListener(
			'ping',
			(e) => {
				const time = new Date(`${JSON.parse(e.data).time}Z`);
				socketStore.update((s) => {
					s.last_ping = time;
					return s;
				});
				// console.log("ping", time);
			},
			false,
		);

		stream.addEventListener(
			'open',
			() => {
				// Connection was opened.
				socketStore.set({ connected: true, last_ping: null });
				console.log('SSE Connected');
			},
			false,
		);

		stream.addEventListener(
			'error',
			() => {
				// Connection was closed.
				socketStore.set({ connected: false, last_ping: null });
				console.log(`Retrying SSE connection in ${Math.round(timeout / 1000)} seconds...`);
				setTimeout(() => {
					openEvents(Math.min(timeout * 2, 5 * 60 * 1000));
				}, timeout);
			},
			false,
		);

		return stream;
	}

	function closeEvents() {
		if (stream) {
			stream.close();
			stream = null;
		}
	}

	function beforeUnload(event: BeforeUnloadEvent) {
		closeEvents();
	}
</script>

<svelte:window on:beforeunload={beforeUnload} />
