<script lang="ts">
	import { api } from '$lib/api';
	import type { Agent } from '$lib/types';
	import AgentCard from '$lib/components/AgentCard.svelte';

	let agents = $state<Agent[]>([]);
	let error = $state<string | null>(null);
	let runInput = $state('');
	let runningAgent = $state<string | null>(null);
	let runResult = $state<string | null>(null);

	async function load() {
		try {
			agents = await api.listAgents();
		} catch {
			error = 'Could not load agents. Is the API running?';
			agents = [
				{
					name: 'git-summarizer',
					description: 'Summarizes recent git activity into a changelog',
					status: 'idle',
					tools: ['git_log', 'git_diff']
				},
				{
					name: 'content-drafter',
					description: 'Drafts blog posts, tweets, and newsletters from context',
					status: 'idle',
					tools: []
				}
			];
		}
	}

	$effect(() => {
		load();
	});

	async function handleRun(name: string) {
		if (!runInput.trim()) return;
		runningAgent = name;
		runResult = null;
		try {
			const result = await api.runAgent(name, runInput);
			runResult = `Started run: ${result.run_id}`;
		} catch (e) {
			runResult = e instanceof Error ? e.message : 'Failed to start agent';
		} finally {
			runningAgent = null;
		}
	}
</script>

<svelte:head>
	<title>Agents | Orchid</title>
</svelte:head>

<div class="max-w-4xl">
	<h1 class="text-2xl font-bold text-gray-100 mb-6">Agents</h1>

	{#if error}
		<div class="rounded-lg border border-yellow-900 bg-yellow-950 p-3 mb-4">
			<p class="text-xs text-yellow-400">{error}</p>
		</div>
	{/if}

	<div class="mb-6">
		<label class="block text-sm text-gray-400 mb-2" for="agent-input">Agent Input</label>
		<div class="flex gap-2">
			<input
				id="agent-input"
				bind:value={runInput}
				placeholder="Enter input for the agent..."
				class="flex-1 rounded-lg border border-gray-700 bg-gray-900 px-3 py-2 text-sm text-gray-200 placeholder-gray-600 focus:border-emerald-600 focus:outline-none"
			/>
		</div>
		{#if runResult}
			<p class="text-xs text-teal-400 mt-2">{runResult}</p>
		{/if}
	</div>

	<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
		{#each agents as agent (agent.name)}
			<AgentCard {agent} onrun={handleRun} />
		{/each}
	</div>

	{#if agents.length === 0 && !error}
		<p class="text-sm text-gray-500 text-center py-12">Loading agents...</p>
	{/if}
</div>
