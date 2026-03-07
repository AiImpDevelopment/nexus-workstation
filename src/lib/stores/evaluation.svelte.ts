// Evaluation Chain Store — Agent-as-a-Judge
import { invoke } from '@tauri-apps/api/core';

interface EvalDimensions {
    correctness: number;
    completeness: number;
    coherence: number;
    safety: number;
    helpfulness: number;
}

interface StageVerdict {
    stage: string;
    score: number;
    dimensions: EvalDimensions;
    reasoning: string;
    issues: string[];
    timestamp: string;
}

interface EvalResult {
    id: string;
    input: string;
    output: string;
    agent_id: string;
    verdicts: StageVerdict[];
    final_score: number;
    passed: boolean;
    threshold: number;
    recommendation: 'accept' | 'revise' | 'reject' | 'escalate';
    timestamp: string;
}

interface EvalConfig {
    threshold_accept: number;
    threshold_revise: number;
    enable_critic: boolean;
    enable_defender: boolean;
    max_retries: number;
    ollama_url: string;
    grader_model: string;
    critic_model: string;
    meta_judge_model: string;
}

class EvaluationStore {
    results = $state<EvalResult[]>([]);
    currentResult = $state<EvalResult | null>(null);
    loading = $state(false);
    config = $state<EvalConfig>({
        threshold_accept: 0.7,
        threshold_revise: 0.4,
        enable_critic: true,
        enable_defender: true,
        max_retries: 2,
        ollama_url: 'http://localhost:11434',
        grader_model: 'dolphin3:8b',
        critic_model: 'hermes3:8b',
        meta_judge_model: 'qwen2.5-coder:7b',
    });

    totalEvaluations = $derived(this.results.length);
    passRate = $derived(
        this.results.length > 0
            ? this.results.filter(r => r.passed).length / this.results.length
            : 0
    );
    averageScore = $derived(
        this.results.length > 0
            ? this.results.reduce((sum, r) => sum + r.final_score, 0) / this.results.length
            : 0
    );

    async evaluate(input: string, output: string, agentId: string): Promise<EvalResult> {
        this.loading = true;
        try {
            const result = await invoke<EvalResult>('eval_agent_output', {
                input,
                output,
                agentId,
                config: this.config,
            });
            this.currentResult = result;
            this.results = [result, ...this.results].slice(0, 100);
            return result;
        } finally {
            this.loading = false;
        }
    }

    async quickEval(input: string, output: string): Promise<StageVerdict> {
        return invoke<StageVerdict>('eval_quick', { input, output });
    }

    async loadHistory(): Promise<void> {
        try {
            this.results = await invoke<EvalResult[]>('eval_history');
        } catch {
            // History not available yet
        }
    }
}

export const evaluation = new EvaluationStore();
export type { EvalResult, EvalConfig, StageVerdict, EvalDimensions };
