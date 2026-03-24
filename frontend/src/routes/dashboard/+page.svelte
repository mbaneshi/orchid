<script lang="ts">
	import { api } from '$lib/api';
	import type { DashboardStats } from '$lib/types';

	let stats = $state<DashboardStats | null>(null);
	let error = $state<string | null>(null);
	let healthy = $state<boolean | null>(null);

	async function load() {
		try {
			const health = await api.health();
			healthy = health.status === 'orchid ok' || health.status === 'ok';
		} catch {
			healthy = false;
		}

		try {
			stats = await api.getDashboardStats();
		} catch {
			error = 'Could not load dashboard stats. Is the Orchid API running on port 3100?';
		}
	}

	$effect(() => {
		load();
	});

	const cards = $derived(
		stats
			? [
					{ label: 'Sessions', value: stats.sessions, color: 'emerald' },
					{ label: 'Agents', value: stats.agents, color: 'teal' },
					{ label: 'Artifacts', value: stats.artifacts, color: 'emerald' },
					{ label: 'Flows', value: stats.flows, color: 'teal' }
				]
			: []
	);
</script>

<svelte:head>
	<title>Dashboard | Orchid</title>
</svelte:head>

<div class="max-w-4xl">
	<div class="mb-6">
		<h1 class="text-2xl font-bold text-gray-100">Dashboard</h1>
		<div class="flex items-center gap-2 mt-1">
			<span
				class="w-2 h-2 rounded-full {healthy === true
					? 'bg-emerald-400'
					: healthy === false
						? 'bg-red-400'
						: 'bg-gray-600'}"
			></span>
			<span class="text-sm text-gray-400">
				{healthy === true ? 'API connected' : healthy === false ? 'API offline' : 'Checking...'}
			</span>
		</div>
	</div>

	{#if error}
		<div class="rounded-lg border border-yellow-900 bg-yellow-950 p-4 mb-6">
			<p class="text-sm text-yellow-300">{error}</p>
			<p class="text-xs text-yellow-600 mt-1">Start the API with: orchid web</p>
		</div>
	{/if}

	{#if stats}
		<div class="grid grid-cols-2 gap-4 mb-8">
			{#each cards as card}
				<div class="rounded-lg border border-gray-800 bg-gray-900 p-4">
					<p class="text-xs text-gray-500 uppercase tracking-wide">{card.label}</p>
					<p class="text-3xl font-bold text-{card.color}-400 mt-1">{card.value}</p>
				</div>
			{/each}
		</div>
	{:else if !error}
		<div class="grid grid-cols-2 gap-4 mb-8">
			{#each Array(4) as _}
				<div class="rounded-lg border border-gray-800 bg-gray-900 p-4 animate-pulse">
					<div class="h-3 w-16 bg-gray-800 rounded mb-2"></div>
					<div class="h-8 w-12 bg-gray-800 rounded"></div>
				</div>
			{/each}
		</div>
	{/if}

	<div class="rounded-lg border border-gray-800 bg-gray-900 p-6">
		<h2 class="text-lg font-semibold text-gray-200 mb-3">Quick Start</h2>
		<div class="space-y-2 text-sm text-gray-400 font-mono">
			<p><span class="text-gray-600">#</span> Run an agent</p>
			<p class="text-emerald-400">orchid agent git-summarizer -r .</p>
			<p class="mt-2"><span class="text-gray-600">#</span> Execute a flow</p>
			<p class="text-emerald-400">orchid flow -n dev-to-content -r .</p>
			<p class="mt-2"><span class="text-gray-600">#</span> Start the web server</p>
			<p class="text-emerald-400">orchid web</p>
		</div>
	</div>
</div>
