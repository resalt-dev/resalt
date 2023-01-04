<script lang="ts">
	import { Card, CardBody, CardHeader, CardTitle, Col, Row } from 'sveltestrap';
	import Icon from '../../components/Icon.svelte';
	import { config, theme } from '../../stores';
</script>

<Row>
	<Col xs="12" class="pb-3">
		<Card class={$theme.dark ? 'bg-dark' : ''}>
			<CardBody>
				<!-- welcome title -->
				<h1 class="display-4">Welcome to Resalt!</h1>
				<!-- subtitle -->
				<p class="lead">This is the control panel for your Resalt instance.</p>
				<br />
				<br />
				<Row>
					<Col xs="12" lg="4">
						<h5 class="card-title">Get Started</h5>
					</Col>
					<Col xs="12" lg="4">
						<h5 class="card-title">Next Steps</h5>
						Hello!
					</Col>
					<Col xs="12" lg="4">
						<h5 class="card-title">More Actions</h5>
						Hello!
					</Col>
				</Row>
			</CardBody>
		</Card>
	</Col>
	<Col xs="12" xl="4" class="pb-3">
		<Card class={$theme.dark ? 'bg-dark' : ''}>
			<CardHeader>
				<CardTitle class="mb-0">System Summary</CardTitle>
			</CardHeader>
			<CardBody>
				<h5 class="card-title">Status</h5>
				<p class="card-text">
					<Icon name="check-circle" class="text-success m-2" /> All systems are operational
				</p>
			</CardBody>
		</Card>
	</Col>
	<Col xs="12" xl="4" class="pb-3">
		<Card class={$theme.dark ? 'bg-dark' : ''}>
			<CardHeader>
				<CardTitle class="mb-0">Quick Links</CardTitle>
			</CardHeader>
			<CardBody class="text-center">
				<a
					href="https://resalt.dev/"
					class="text-reset text-decoration-none py-4"
					target="_blank"
					rel="noopener noreferrer"
				>
					<Icon name="buildings" size="7" class="mb-3" />
					<h5 class="card-title">Go to Documentation</h5>
					<p class="card-text">Find documentation, guides, and more</p>
				</a>

				<hr class="text-light" />

				<a
					href="https://discord.resalt.dev/"
					class="text-reset text-decoration-none py-4"
					target="_blank"
					rel="noopener noreferrer"
				>
					<Icon name="comment-dots" size="7" class="mb-3" />
					<h5 class="card-title">Give Feedback</h5>
					<p class="card-text">Give feedback or report a bug</p>
				</a>
			</CardBody>
		</Card>
	</Col>
	<Col xs="12" xl="4" class="pb-3">
		<Card class={$theme.dark ? 'bg-dark' : ''}>
			<CardHeader>
				<CardTitle class="mb-0">Latest News</CardTitle>
			</CardHeader>
			<CardBody>
				{#each $config.latestNews as news, i}
					{#if i !== 0}
						<hr class="text-light" />
					{/if}
					<h5 class="card-title">{news.split('§')[0]}</h5>
					{#if news.split('§')[1]}
						<p class="card-text">{news.split('§')[1]}</p>
					{/if}
				{/each}
			</CardBody>
		</Card>
	</Col>
</Row>
