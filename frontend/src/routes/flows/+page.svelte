<script lang="ts">
	import { api } from '$lib/api';
	import type { Workflow } from '$lib/types';
	import FlowRunner from '$lib/components/FlowRunner.svelte';

	let workflows = $state<Workflow[]>([]);
	let error = $state<string | null>(null);

	async function load() {
		try {
			workflows = await api.listFlows();
		} catch {
			error = 'Could not load flows. Is the API running?';
			workflows = [
				{
					id: 'demo-flow-1',
					name: 'dev-to-content',
					trigger: { type: 'Manual' },
					steps: [
						{
							id: 'step-1',
							name: 'Summarize Git',
							agent_name: 'git-summarizer',
							depends_on: []
						},
						{
							id: 'step-2',
							name: 'Draft Content',
							agent_name: 'content-drafter',
							depends_on: ['step-1']
						}
					]
				}
			];
		}
	}

	$effect(() => {
		load();
	});
</script>

<svelte:head>
	<title>Flows | Orchid</title>
</svelte:head>

<div class="max-w-4xl">
	<h1 class="text-2xl font-bold text-gray-100 mb-6">Flows</h1>

	{#if error}
		<div class="rounded-lg border border-yellow-900 bg-yellow-950 p-3 mb-4">
			<p class="text-xs text-yellow-400">{error}</p>
		</div>
	{/if}

	<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
		{#each workflows as workflow (workflow.id)}
			<FlowRunner {workflow} />
		{/each}
	</div>

	{#if workflows.length === 0 && !error}
		<p class="text-sm text-gray-500 text-center py-12">Loading flows...</p>
	{/if}
</div>
