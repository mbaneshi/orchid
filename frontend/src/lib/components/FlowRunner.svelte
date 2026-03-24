<script lang="ts">
	import type { Workflow, FlowRun } from '$lib/types';
	import { api } from '$lib/api';
	import StatusBadge from './StatusBadge.svelte';

	interface Props {
		workflow: Workflow;
	}

	let { workflow }: Props = $props();

	let running = $state(false);
	let currentRun = $state<FlowRun | null>(null);
	let error = $state<string | null>(null);

	async function run() {
		running = true;
		error = null;
		try {
			currentRun = await api.runFlow(workflow.id);
			pollStatus();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Unknown error';
			running = false;
		}
	}

	async function pollStatus() {
		if (!currentRun) return;
		try {
			const updated = await api.getFlowRun(currentRun.id);
			currentRun = updated;
			if (updated.status === 'running' || updated.status === 'pending') {
				setTimeout(pollStatus, 2000);
			} else {
				running = false;
			}
		} catch {
			running = false;
		}
	}

	function triggerLabel(t: Workflow['trigger']): string {
		switch (t.type) {
			case 'Manual': return 'Manual';
			case 'Schedule': return `Cron: ${t.cron}`;
			case 'Webhook': return `Webhook: ${t.path}`;
		}
	}
</script>

<div class="rounded-lg border border-gray-800 bg-gray-900 p-4">
	<div class="flex items-center justify-between mb-3">
		<h3 class="font-semibold text-gray-100">{workflow.name}</h3>
		<span class="text-xs text-gray-500">{triggerLabel(workflow.trigger)}</span>
	</div>

	<div class="space-y-1 mb-4">
		{#each workflow.steps as step, i}
			<div class="flex items-center gap-2 text-xs text-gray-400">
				<span class="w-5 h-5 rounded-full bg-gray-800 flex items-center justify-center text-gray-500 shrink-0">
					{i + 1}
				</span>
				<span>{step.name}</span>
				<span class="text-gray-600">({step.agent_name})</span>
			</div>
		{/each}
	</div>

	{#if currentRun}
		<div class="mb-3 p-2 rounded bg-gray-950 border border-gray-800">
			<div class="flex items-center justify-between text-xs">
				<span class="text-gray-400">Run {currentRun.id.slice(0, 8)}</span>
				<StatusBadge status={currentRun.status} />
			</div>
			{#if currentRun.output}
				<pre class="text-xs text-gray-400 mt-2 whitespace-pre-wrap">{currentRun.output}</pre>
			{/if}
		</div>
	{/if}

	{#if error}
		<p class="text-xs text-red-400 mb-2">{error}</p>
	{/if}

	<button
		onclick={run}
		disabled={running}
		class="w-full py-2 text-sm font-medium rounded transition-colors
			{running
			? 'bg-gray-700 text-gray-400 cursor-not-allowed'
			: 'bg-teal-700 hover:bg-teal-600 text-white'}"
	>
		{running ? 'Running...' : 'Run Flow'}
	</button>
</div>
