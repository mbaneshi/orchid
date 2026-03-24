<script lang="ts">
	import type { Agent } from '$lib/types';
	import StatusBadge from './StatusBadge.svelte';

	interface Props {
		agent: Agent;
		onrun?: (name: string) => void;
	}

	let { agent, onrun }: Props = $props();
</script>

<div class="rounded-lg border border-gray-800 bg-gray-900 p-4 hover:border-emerald-800 transition-colors">
	<div class="flex items-center justify-between mb-2">
		<h3 class="text-sm font-semibold text-gray-100">{agent.name}</h3>
		<StatusBadge status={agent.status} />
	</div>
	<p class="text-xs text-gray-400 mb-3">{agent.description}</p>
	{#if agent.tools.length > 0}
		<div class="flex flex-wrap gap-1 mb-3">
			{#each agent.tools as tool}
				<span class="px-1.5 py-0.5 text-xs rounded bg-gray-800 text-gray-400">{tool}</span>
			{/each}
		</div>
	{/if}
	{#if onrun}
		<button
			onclick={() => onrun?.(agent.name)}
			class="w-full py-1.5 text-xs font-medium rounded bg-emerald-700 hover:bg-emerald-600 text-white transition-colors"
		>
			Run
		</button>
	{/if}
</div>
