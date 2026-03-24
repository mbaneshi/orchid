export interface Session {
	id: string;
	name: string;
	created_at: string;
	updated_at: string;
}

export interface CodeSession extends Session {
	repo_path?: string;
	branch?: string;
}

export interface ContentSession extends Session {
	content_type: string;
	draft?: string;
}

export interface RelationshipSession extends Session {
	contact_name?: string;
	context?: string;
}

export interface Agent {
	name: string;
	description: string;
	status: AgentStatus;
	tools: string[];
}

export type AgentStatus = 'idle' | 'running' | 'error' | 'done';

export interface Artifact {
	id: string;
	session_id?: string;
	artifact_type: string;
	content: string;
	metadata?: string;
	created_at: string;
}

export interface Workflow {
	id: string;
	name: string;
	trigger: Trigger;
	steps: Step[];
}

export type Trigger =
	| { type: 'Manual' }
	| { type: 'Schedule'; cron: string }
	| { type: 'Webhook'; path: string };

export interface Step {
	id: string;
	name: string;
	agent_name: string;
	depends_on: string[];
}

export interface FlowRun {
	id: string;
	workflow_id: string;
	status: FlowRunStatus;
	started_at: string;
	finished_at?: string;
	output?: string;
}

export type FlowRunStatus = 'pending' | 'running' | 'completed' | 'failed';

export interface HealthResponse {
	status: string;
}

export interface DashboardStats {
	sessions: number;
	agents: number;
	artifacts: number;
	flows: number;
}
