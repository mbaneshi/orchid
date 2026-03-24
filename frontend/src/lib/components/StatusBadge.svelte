<script lang="ts">
	import type { AgentStatus, FlowRunStatus } from '$lib/types';

	type Status = AgentStatus | FlowRunStatus;

	interface Props {
		status: Status;
	}

	let { status }: Props = $props();

	const colors: Record<Status, string> = {
		idle: 'bg-gray-700 text-gray-300',
		running: 'bg-teal-900 text-teal-300 animate-pulse',
		error: 'bg-red-900 text-red-300',
		done: 'bg-emerald-900 text-emerald-300',
		pending: 'bg-gray-700 text-gray-300',
		completed: 'bg-emerald-900 text-emerald-300',
		failed: 'bg-red-900 text-red-300'
	};
</script>

<span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium {colors[status]}">
	{status}
</span>
