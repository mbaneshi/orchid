import type {
	Agent,
	Artifact,
	DashboardStats,
	FlowRun,
	HealthResponse,
	Session,
	Workflow
} from './types';

const BASE_URL = '/api';

async function request<T>(path: string, init?: RequestInit): Promise<T> {
	const res = await fetch(`${BASE_URL}${path}`, {
		headers: { 'Content-Type': 'application/json' },
		...init
	});
	if (!res.ok) {
		throw new Error(`API error: ${res.status} ${res.statusText}`);
	}
	return res.json() as Promise<T>;
}

export const api = {
	health(): Promise<HealthResponse> {
		return request('/health');
	},

	getDashboardStats(): Promise<DashboardStats> {
		return request('/dashboard/stats');
	},

	// Sessions
	listSessions(): Promise<Session[]> {
		return request('/sessions');
	},
	getSession(id: string): Promise<Session> {
		return request(`/sessions/${id}`);
	},

	// Agents
	listAgents(): Promise<Agent[]> {
		return request('/agents');
	},
	runAgent(name: string, input: string): Promise<{ run_id: string }> {
		return request(`/agents/${name}/run`, {
			method: 'POST',
			body: JSON.stringify({ input })
		});
	},

	// Artifacts
	listArtifacts(type?: string): Promise<Artifact[]> {
		const query = type ? `?type=${encodeURIComponent(type)}` : '';
		return request(`/artifacts${query}`);
	},
	getArtifact(id: string): Promise<Artifact> {
		return request(`/artifacts/${id}`);
	},

	// Flows / Workflows
	listFlows(): Promise<Workflow[]> {
		return request('/flows');
	},
	getFlow(id: string): Promise<Workflow> {
		return request(`/flows/${id}`);
	},
	runFlow(id: string): Promise<FlowRun> {
		return request(`/flows/${id}/run`, { method: 'POST' });
	},
	getFlowRun(id: string): Promise<FlowRun> {
		return request(`/flows/runs/${id}`);
	}
};
