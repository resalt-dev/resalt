<script lang="ts">
	import { theme } from '$lib/stores';
</script>

<div class="progress" role="progressbar" aria-label="Loading" aria-valuenow={100}>
	<div class="progress-bar progress-bar-animated bg-{$theme.color}" style="width: 100%">
		Loading...
	</div>
</div>
