<script lang="ts">
	import { theme } from '$lib/stores';
</script>

<div class="progress" role="progressbar">
	<div
		class="progress-bar progress-bar-striped progress-bar-animated bg-{$theme.color}"
		style="width: 100%"
	>
		Loading...
	</div>
</div>
