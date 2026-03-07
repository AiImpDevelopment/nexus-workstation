# Agent-as-a-Judge: Comprehensive Research for Nexus Evaluation Pipeline

**Date:** 2026-03-07
**Purpose:** Research foundation for implementing Agent-as-a-Judge evaluation in the Nexus Tauri desktop application
**Scope:** Paper analysis, framework comparison, local model integration, implementation architecture

---

## Table of Contents

1. [Paper Summaries](#1-paper-summaries)
2. [Frameworks and Methodologies](#2-frameworks-and-methodologies)
3. [Architecture Design](#3-architecture-design)
4. [Local Model Implementation (Ollama)](#4-local-model-implementation-ollama)
5. [Multi-Agent Evaluation Pipeline](#5-multi-agent-evaluation-pipeline)
6. [Metrics and Scoring Rubrics](#6-metrics-and-scoring-rubrics)
7. [Claude Code Hooks Integration](#7-claude-code-hooks-integration)
8. [Implementation Recommendations for Nexus](#8-implementation-recommendations-for-nexus)
9. [Morpheus Tutorials Reference](#9-morpheus-tutorials-reference)
10. [Sources](#10-sources)

---

## 1. Paper Summaries

### 1.1 "Agent-as-a-Judge: Evaluate Agents with Agents" (arXiv 2410.10934)

**Authors:** Mingchen Zhuge, Changsheng Zhao, Dylan Ashley, Wenyi Wang, Dmitrii Khizbullin, Yunyang Xiong, Zechun Liu, Ernie Chang, Raghuraman Krishnamoorthi, Yuandong Tian, Yangyang Shi, Vikas Chandra, Juergen Schmidhuber
**Published:** October 2024

**Core Contribution:** Introduces the Agent-as-a-Judge paradigm -- an extension of LLM-as-a-Judge that incorporates agentic features (tool use, code execution, web browsing) to enable intermediate feedback during evaluation.

**Key Findings:**
- Agent-as-a-Judge **dramatically outperforms** LLM-as-a-Judge in evaluation reliability
- Performance matches human evaluation baselines
- Provides rich reward signals enabling agent self-improvement loops
- Single-pass LLM judging misses execution context, tool correctness, and intermediate states

**Benchmark -- DevAI:**
- 55 realistic automated AI development tasks
- 365 hierarchical user requirements with manual annotations
- Tests three popular agentic systems end-to-end
- Evaluates code generation, debugging, testing, and deployment

**Key Insight:** The judge agent must have the SAME capabilities as the agent being evaluated (tool access, code execution, environment interaction) to provide meaningful assessment.

```
Evaluation Paradigm Evolution:
  Human Judge      --> Expensive, slow, gold standard
  LLM-as-Judge     --> Fast, cheap, misses execution context
  Agent-as-Judge   --> Agentic capabilities, intermediate feedback, matches human quality
```

---

### 1.2 "A Survey on Agent-as-a-Judge" (arXiv 2601.05111)

**Authors:** Runyang You, Hongru Cai, Caiqi Zhang, Qiancheng Xu, Meng Liu, Tiezheng Yu, Yongqi Li, Wenjie Li
**Published:** January 2026

**Core Contribution:** First comprehensive survey tracing the evolution from LLM-as-a-Judge to Agent-as-a-Judge, establishing a developmental taxonomy.

**Taxonomic Framework -- Four Key Dimensions:**

| Dimension | LLM-as-Judge | Agent-as-Judge |
|-----------|-------------|----------------|
| Planning | None / single-pass | Multi-step reasoning, task decomposition |
| Verification | Text-only analysis | Tool-based execution and validation |
| Collaboration | Single model | Multi-agent teamwork, debate, consensus |
| Memory | Stateless | Persistent memory across evaluations |

**Application Domains Surveyed:**
- General NLP tasks (summarization, translation, dialogue)
- Code generation and software engineering
- Scientific reasoning and mathematical proofs
- Medical diagnosis validation
- Legal document analysis
- Educational assessment

**Future Research Directions:**
1. Reducing bias through multi-perspective evaluation
2. Scaling to complex real-world tasks
3. Meta-evaluation (evaluating the evaluators)
4. Cross-domain generalization
5. Efficiency vs. accuracy trade-offs for local deployment

---

### 1.3 "Multi-Agent-as-Judge" (arXiv 2507.21028)

**Authors:** Jiaju Chen, Yuxuan Lu, Xiaojie Wang, Huimin Zeng, Jing Huang, Jiri Gesi, Ying Xu, Bingsheng Yao, Dakuo Wang
**Published:** July 2025

**Core Contribution:** Introduces MAJ-EVAL, a framework that automatically constructs multiple evaluator personas from domain documents and uses multi-agent debate for evaluation.

**Architecture -- MAJ-EVAL:**
```
                    Domain Documents
                    (Papers, Guides)
                          |
                    [Persona Extractor]
                    /        |        \
          Persona A     Persona B     Persona C
         (Accuracy)   (Completeness) (Pedagogy)
                \          |          /
                 [Multi-Agent Debate]
                          |
                  [Consensus Score]
                          |
                  Multi-Dimensional
                     Feedback
```

**Key Innovation -- Automated Persona Construction:**
- Instead of manually designing evaluator roles, extract them from relevant documents
- Each persona embodies a distinct evaluation dimension
- Tested across educational and medical domains
- Better alignment with human expert ratings than conventional LLM-as-Judge

**Methodology:**
1. Extract evaluator personas from domain-specific text documents
2. Instantiate LLM agents embodying these personas
3. Facilitate structured multi-agent debates
4. Generate multi-dimensional feedback aligned with human perspectives
5. Produce consensus scores through deliberation

---

### 1.4 "When AIs Judge AIs" (arXiv 2508.02994)

**Author:** Fangyi Yu
**Published:** August 2025

**Core Contribution:** Review paper mapping the evolution from single evaluators to multi-agent debate systems, examining trade-offs across reliability, cost, and human alignment.

**Taxonomy of Approaches:**

| Approach | Complexity | Cost | Reliability |
|----------|-----------|------|-------------|
| Single LLM Judge | Low | Low | Moderate |
| Multi-LLM Panel | Medium | Medium | Good |
| Agent-as-Judge | High | High | Very Good |
| Multi-Agent Debate | Very High | Very High | Excellent |

**Sector-Specific Analysis:**
- **Medicine:** High-stakes, requires domain expertise in judges
- **Law:** Needs precedent awareness and nuanced reasoning
- **Finance:** Requires numerical accuracy verification
- **Education:** Must assess pedagogical quality, not just correctness

**Critical Research Challenges:**
1. Bias mitigation in automated judges
2. Robustness testing of evaluation pipelines
3. Meta-evaluation approaches (who judges the judges?)
4. Balancing automation with human oversight
5. Cost-effectiveness for production deployment

**Key Recommendation:** AI judgment should ENHANCE, not REPLACE, human oversight. Hybrid approaches combining automated evaluation with targeted human review yield the best results.

---

### 1.5 "Build, Judge, Optimize" (arXiv 2603.03565)

**Authors:** Alejandro Breen Herrera, Aayush Sheth, Steven G. Xu, Zhucheng Zhan, Charles Wright, Marcus Yearwood, Hongtai Wei, Sudeep Das
**Published:** March 2026

**Core Contribution:** A practical blueprint for continuous improvement of multi-agent consumer assistants through structured evaluation and prompt optimization.

**Three-Phase Architecture:**

```
Phase 1: BUILD                    Phase 2: JUDGE                    Phase 3: OPTIMIZE
+-------------------+            +-------------------+            +-------------------+
| Multi-Agent       |            | Multi-Faceted     |            | Prompt            |
| Shopping          |  ------>>  | Evaluation        |  ------>>  | Optimization      |
| Assistant         |            | Rubric            |            | (GEPA)            |
+-------------------+            +-------------------+            +-------------------+
| - Sub-agents      |            | - Structured      |            | - Sub-agent GEPA  |
| - Tool calling    |            |   dimensions      |            |   (local rubrics) |
| - Multi-turn      |            | - LLM-as-judge    |            | - MAMuT GEPA      |
| - Context mgmt    |            |   calibrated w/   |            |   (trajectory-    |
|                   |            |   human labels    |            |    level scoring) |
+-------------------+            +-------------------+            +-------------------+
```

**Key Innovations:**
1. **Multi-faceted evaluation rubric** decomposing quality into structured, measurable dimensions
2. **LLM-as-judge pipeline** calibrated against human annotations for reliability
3. **Sub-agent GEPA:** Optimizes individual agent nodes against localized rubrics
4. **MAMuT GEPA (Multi-Agent Multi-Turn):** Jointly optimizes prompts across agents using multi-turn simulation and trajectory-level scoring

**Production Relevance:** This is the most directly applicable paper for Nexus -- it provides actionable templates for building, evaluating, and continuously improving multi-agent systems in production.

---

## 2. Frameworks and Methodologies

### 2.1 DeepEval Framework

DeepEval is an open-source LLM evaluation framework (20M+ evaluations run) providing unit-test-style evaluation for LLM applications.

**Agent Evaluation Architecture -- Three Layers:**

```
+------------------------------------------------------------------+
|                     REASONING LAYER                               |
|  +--------------------+    +--------------------+                |
|  | Plan Quality       |    | Plan Adherence     |                |
|  | (strategy logic)   |    | (execution fidelity)|               |
|  +--------------------+    +--------------------+                |
+------------------------------------------------------------------+
|                      ACTION LAYER                                 |
|  +--------------------+    +--------------------+                |
|  | Tool Correctness   |    | Argument           |                |
|  | (selection/order)  |    | Correctness        |                |
|  +--------------------+    +--------------------+                |
+------------------------------------------------------------------+
|                    EXECUTION LAYER                                |
|  +--------------------+    +--------------------+                |
|  | Task Completion    |    | Step Efficiency    |                |
|  | (goal achievement) |    | (resource usage)   |                |
|  +--------------------+    +--------------------+                |
+------------------------------------------------------------------+
```

**Six Core Agent Metrics:**

| Metric | Layer | What it Measures | Scoring |
|--------|-------|-----------------|---------|
| Plan Quality | Reasoning | Logic, completeness, efficiency of plans | LLM-evaluated |
| Plan Adherence | Reasoning | Deviation from stated plan during execution | LLM-evaluated |
| Tool Correctness | Action | Proper tool selection and invocation | Ratio-based + strictness levels |
| Argument Correctness | Action | Parameter quality vs. task requirements | LLM-evaluated, referenceless |
| Task Completion | Execution | Goal achievement (1.0 = full success) | LLM-evaluated vs. requirements |
| Step Efficiency | Execution | Unnecessary steps, redundant tool calls | LLM-evaluated penalty system |

**Implementation Pattern:**
```python
from deepeval import evaluate
from deepeval.metrics import (
    ToolCorrectnessMetric,
    TaskCompletionMetric,
    StepEfficiencyMetric
)
from deepeval.test_case import LLMTestCase

# Trace-based evaluation with @observe decorators
@observe(type="agent")       # Top-level orchestration
@observe(type="llm")         # Reasoning decisions
@observe(type="tool")        # Action execution

# Metric selection guide:
# Planning-heavy agents     --> Plan Quality + Plan Adherence
# Tool-dependent agents     --> Tool Correctness + Argument Correctness
# Multi-step workflows      --> Step Efficiency + Task Completion
# Cost-sensitive production --> Prioritize Step Efficiency
```

**Ollama Integration:**
DeepEval supports ANY LLM judge including Ollama. Users can swap OpenAI for local models without code changes -- only configuration changes needed.

---

### 2.2 G-Eval Methodology

**Origin:** "G-Eval: NLG Evaluation using GPT-4 with Better Human Alignment" (arXiv 2303.16634, EMNLP 2023)

**Three-Phase Process:**

```
Phase 1: EVALUATION STEP GENERATION
+---------------------------+
| Task Introduction         |
| + Evaluation Criteria     |
|         |                 |
|    [LLM + CoT]            |
|         |                 |
|  Structured Eval Steps    |
+---------------------------+

Phase 2: JUDGING
+---------------------------+
| Eval Steps + NLG Output   |
|         |                 |
|    [LLM Judge]            |
|         |                 |
|   Raw Score + Reasoning   |
+---------------------------+

Phase 3: SCORING
+---------------------------+
| Raw Scores                |
| + Log-Probabilities       |
|         |                 |
|  [Weighted Aggregation]   |
|         |                 |
|   Final G-Eval Score      |
+---------------------------+
```

**Performance:** GPT-4 backbone achieves Spearman correlation of 0.514 with humans on summarization -- outperforming ALL previous automated metrics (BLEU, ROUGE, BERTScore).

**Key Innovation:** Uses chain-of-thought prompting to generate evaluation steps, then weighs scores by log-probabilities for calibrated output. No reference text needed.

**Adaptation for Local Models:** G-Eval's three-phase approach can be replicated with Ollama models, though log-probability access varies by model and backend.

---

### 2.3 Prometheus -- Open-Source Evaluation LLM

**Origin:** "Prometheus: Inducing Fine-grained Evaluation Capability in Language Models" (ICLR 2024)

**Key Features:**
- Fine-tuned specifically for evaluation tasks
- 7B and 8x7B parameter versions available
- Pearson correlation of 0.897 with human evaluators (on par with GPT-4 at 0.882)
- Supports custom score rubrics
- 7B version runs on 16GB VRAM (fits RX 7800 XT)

**Prometheus 2 Improvements:**
- Handles both direct assessment AND pairwise ranking
- 7B version outperforms Llama-2-70B as a judge
- Open weights, fully reproducible

**Local Deployment Path:**
```bash
# Via Ollama (if GGUF available)
ollama run prometheus-2:7b

# Via vLLM (recommended by Prometheus team)
python -m vllm.entrypoints.openai.api_server \
    --model prometheus-eval/prometheus-7b-v2.0 \
    --port 8100
```

---

### 2.4 LLM-as-Judge Best Practices (2025-2026 Consensus)

**Seven Critical Best Practices:**

| # | Practice | Implementation |
|---|----------|---------------|
| 1 | **Combine automated + human review** | Use LLM judge at scale, flag edge cases for human review |
| 2 | **Mitigate position bias** | Swap response positions, only count consistent wins |
| 3 | **Mitigate verbosity bias** | Explicit rubric: "Penalize unnecessarily verbose responses" |
| 4 | **Chain-of-thought reasoning** | Force judge to explain reasoning BEFORE scoring |
| 5 | **Calibrate with golden dataset** | Maintain human-labeled test set, measure agreement drift |
| 6 | **Hide model identity** | Prevent self-evaluation and authority bias |
| 7 | **Use structured rubrics** | Provide explicit score definitions for each point on the scale |

**Known Biases to Mitigate:**

| Bias | Description | Mitigation |
|------|-------------|-----------|
| Position Bias | Favors first/last response (~40% GPT-4 inconsistency) | Swap ordering, require consistent preference |
| Verbosity Bias | Longer = better (false assumption) | Explicit anti-verbosity rubric instructions |
| Self-Enhancement | Models prefer their own outputs | Hide model identity, use different judge model |
| Authority Bias | Known model names get higher scores | Anonymous evaluation |
| Leniency Bias | Avoiding low scores | Calibrate with known-bad examples |

---

## 3. Architecture Design

### 3.1 Nexus Agent Evaluation Architecture (Full System)

```
+============================================================================+
|                        NEXUS TAURI DESKTOP APP                              |
|                                                                            |
|  +------------------+  +------------------+  +------------------+          |
|  |   Agent Runner   |  |  Evaluation UI   |  | Results Dashboard|          |
|  |   (Rust/Tauri)   |  |  (React/Solid)   |  |  (Charts/Tables) |          |
|  +--------+---------+  +--------+---------+  +--------+---------+          |
|           |                     |                     |                     |
+===========|=====================|=====================|=====================+
            |                     |                     |
            v                     v                     v
+===========================================================================+
|                      EVALUATION ORCHESTRATOR                               |
|                         (Rust Backend)                                      |
|                                                                            |
|  +-------------------+  +-------------------+  +-------------------+       |
|  | Task Decomposer   |  | Rubric Engine     |  | Score Aggregator  |       |
|  | (breaks task into |  | (loads/generates  |  | (weighted merge   |       |
|  |  eval dimensions) |  |  scoring rubrics) |  |  + calibration)   |       |
|  +-------------------+  +-------------------+  +-------------------+       |
|                                                                            |
+==================================|=========================================+
                                   |
                    +--------------+--------------+
                    |              |              |
                    v              v              v
+===============+  +============+  +==============+
| LOCAL JUDGE   |  | CLOUD JUDGE|  | HUMAN REVIEW |
| (Ollama)      |  | (Claude)   |  | (UI Panel)   |
|               |  |            |  |              |
| - hermes-3   |  | - opus-4   |  | - Flagged    |
| - qwen-coder |  | - sonnet   |  |   cases      |
| - prometheus  |  |            |  | - Calibration|
| - dolphin-3  |  |            |  |   samples    |
+===============+  +============+  +==============+
        |                |                |
        v                v                v
+===========================================================================+
|                       RESULTS STORE                                        |
|  +-------------------+  +-------------------+  +-------------------+       |
|  | PostgreSQL        |  | Evaluation History|  | Drift Detection   |       |
|  | (scores, traces)  |  | (temporal trends) |  | (golden set delta)|       |
|  +-------------------+  +-------------------+  +-------------------+       |
+===========================================================================+
```

### 3.2 Multi-Agent Judge Panel Architecture

```
+------------------------------------------------------------------+
|                    EVALUATION REQUEST                              |
|   Input: Agent output + Task description + Context                |
+----------------------------------+-------------------------------+
                                   |
                    +--------------+--------------+
                    |              |              |
                    v              v              v
          +-----------+    +-----------+    +-----------+
          | JUDGE A   |    | JUDGE B   |    | JUDGE C   |
          | Accuracy  |    | Complete- |    | Style &   |
          | & Correct-|    | ness &    |    | Efficiency|
          | ness      |    | Coverage  |    |           |
          | (hermes-3)|    | (qwen-cod)|    | (dolphin) |
          +-----+-----+    +-----+-----+    +-----+-----+
                |                |                |
                v                v                v
          [Score: 0.85]    [Score: 0.72]    [Score: 0.91]
          [Reasoning]      [Reasoning]      [Reasoning]
                |                |                |
                +--------+-------+--------+-------+
                         |                |
                         v                v
              +-------------------+  +-------------------+
              | DEBATE ROUND      |  | CONSENSUS CHECK   |
              | (if scores differ |  | (agreement > 0.8  |
              |  by > 0.2)        |  |  = accept)        |
              +-------------------+  +-------------------+
                         |
                         v
              +-------------------+
              | FINAL VERDICT     |
              | Score: 0.83       |
              | Confidence: 0.91  |
              | Dimensions: {...} |
              +-------------------+
```

### 3.3 Evaluation Pipeline Flow

```
Agent Task Execution
        |
        v
[1. TRACE CAPTURE]
  - Record all tool calls, LLM responses, intermediate states
  - Store execution timeline with timestamps
        |
        v
[2. DIMENSION EXTRACTION]
  - Decompose task into evaluable dimensions
  - Load or generate appropriate rubrics
  - Map dimensions to judge personas
        |
        v
[3. PARALLEL EVALUATION]
  - Each judge evaluates assigned dimensions independently
  - Local judges (Ollama) for cost-sensitive dimensions
  - Cloud judges (Claude) for complex reasoning dimensions
        |
        v
[4. DEBATE & CONSENSUS] (if multi-agent)
  - Compare scores across judges
  - Trigger debate on disagreements (delta > threshold)
  - Reach consensus through structured argumentation
        |
        v
[5. CALIBRATION]
  - Compare against golden dataset scores
  - Apply drift correction if needed
  - Flag for human review if confidence < threshold
        |
        v
[6. SCORE & FEEDBACK]
  - Final weighted score per dimension
  - Aggregate score with confidence interval
  - Actionable feedback for agent improvement
  - Store in PostgreSQL for trend analysis
        |
        v
[7. OPTIMIZATION LOOP]
  - Feed scores back into prompt optimization (GEPA pattern)
  - Track improvement over time
  - Trigger re-evaluation after prompt changes
```

---

## 4. Local Model Implementation (Ollama)

### 4.1 Judge Model Selection for Nexus

| Model | Role | VRAM | Strength | Weakness |
|-------|------|------|----------|----------|
| hermes-3:8b | Reasoning Judge | CPU | Strong logical analysis | Slow on CPU |
| qwen2.5-coder:7b | Code Judge | ~8.5GB GPU | Excellent code understanding | Limited non-code eval |
| dolphin-3:8b | Style/General Judge | CPU | Instruction following | Less precise scoring |
| llava:7b | Visual Judge | ~7GB GPU | Can evaluate UI screenshots | Limited text analysis |
| prometheus-2:7b | Dedicated Evaluator | ~7GB GPU | Trained specifically for judging | Needs GGUF conversion |

### 4.2 Ollama Judge Configuration

```python
# judge_config.py -- Nexus Agent Evaluation Configuration

JUDGE_MODELS = {
    "accuracy": {
        "model": "hermes-3:8b",
        "device": "cpu",
        "temperature": 0.1,  # Low temp for consistent scoring
        "num_predict": 1024,
        "system_prompt": """You are an expert evaluator. Assess the given output
for factual accuracy and correctness. Use the provided rubric to score on a
1-5 scale. Always explain your reasoning BEFORE giving the score."""
    },
    "code_quality": {
        "model": "qwen2.5-coder:7b-instruct-q4_K_M",
        "device": "gpu",
        "temperature": 0.1,
        "num_predict": 2048,
        "system_prompt": """You are a senior code reviewer. Evaluate the code
for correctness, style, efficiency, and security. Score each dimension
independently on a 1-5 scale with explicit justification."""
    },
    "completeness": {
        "model": "dolphin3:8b-llama3.1-q4_K_M",
        "device": "cpu",
        "temperature": 0.2,
        "num_predict": 1024,
        "system_prompt": """You are a quality assurance expert. Assess whether
the output fully addresses all requirements. Check for missing elements,
partial implementations, and edge cases."""
    }
}

EVALUATION_CONFIG = {
    "debate_threshold": 0.2,       # Trigger debate if score delta > 0.2
    "consensus_threshold": 0.8,    # Accept if agreement > 0.8
    "human_review_threshold": 0.5, # Flag for human review if confidence < 0.5
    "max_debate_rounds": 3,        # Max rounds of multi-agent debate
    "golden_set_check_interval": 50 # Re-calibrate every 50 evaluations
}
```

### 4.3 Ollama API Integration

```python
import httpx
import json

OLLAMA_URL = "http://localhost:11434"

async def evaluate_with_ollama(
    model: str,
    prompt: str,
    system: str,
    temperature: float = 0.1
) -> dict:
    """Run a single judge evaluation via Ollama."""
    async with httpx.AsyncClient(timeout=120.0) as client:
        response = await client.post(
            f"{OLLAMA_URL}/api/generate",
            json={
                "model": model,
                "prompt": prompt,
                "system": system,
                "temperature": temperature,
                "stream": False,
                "options": {
                    "num_predict": 2048,
                    "top_p": 0.9,
                    "repeat_penalty": 1.1
                }
            }
        )
        result = response.json()
        return parse_judge_response(result["response"])

def parse_judge_response(text: str) -> dict:
    """Extract structured score and reasoning from judge output."""
    # Expected format:
    # REASONING: <explanation>
    # SCORE: <1-5>
    # DIMENSION_SCORES: {"accuracy": 4, "completeness": 3, ...}
    lines = text.strip().split("\n")
    result = {"raw_text": text, "score": 0, "reasoning": "", "dimensions": {}}

    for line in lines:
        if line.startswith("SCORE:"):
            try:
                result["score"] = float(line.split(":")[1].strip())
            except ValueError:
                result["score"] = 0
        elif line.startswith("REASONING:"):
            result["reasoning"] = line.split(":", 1)[1].strip()
        elif line.startswith("DIMENSION_SCORES:"):
            try:
                result["dimensions"] = json.loads(line.split(":", 1)[1].strip())
            except json.JSONDecodeError:
                pass

    return result
```

### 4.4 G-Eval Adaptation for Ollama

```python
async def g_eval_local(
    task_description: str,
    criteria: str,
    output_to_evaluate: str,
    model: str = "hermes-3:8b"
) -> dict:
    """G-Eval methodology adapted for local Ollama models."""

    # Phase 1: Generate evaluation steps via CoT
    step_generation_prompt = f"""Task: {task_description}
Evaluation Criteria: {criteria}

Generate a detailed, numbered list of evaluation steps to assess the output
against the criteria. Be specific and measurable."""

    steps_response = await evaluate_with_ollama(
        model=model,
        prompt=step_generation_prompt,
        system="You are an evaluation methodology expert. Generate clear, actionable evaluation steps.",
        temperature=0.3
    )

    # Phase 2: Apply steps to evaluate output
    evaluation_prompt = f"""Task: {task_description}
Evaluation Criteria: {criteria}
Evaluation Steps:
{steps_response['raw_text']}

Output to Evaluate:
{output_to_evaluate}

Follow the evaluation steps above to assess this output.
For each step, provide your assessment.
Then give a final score from 1-5.

Format:
STEP_ASSESSMENTS: <your step-by-step assessment>
REASONING: <overall reasoning>
SCORE: <1-5>"""

    return await evaluate_with_ollama(
        model=model,
        prompt=evaluation_prompt,
        system="You are a rigorous evaluator. Follow the steps precisely and score objectively.",
        temperature=0.1
    )
```

---

## 5. Multi-Agent Evaluation Pipeline

### 5.1 Pipeline Architecture for Nexus

```
+===========================================================================+
|                  NEXUS MULTI-AGENT EVALUATION PIPELINE                     |
|                                                                            |
|  INPUT: Agent trace (tool calls, LLM responses, outputs, errors)           |
|                                                                            |
|  STAGE 1: DIMENSION ANALYSIS                                               |
|  +---------------------------------------------------------------+        |
|  | Parse trace --> Extract evaluable components:                  |        |
|  |   - Code outputs    - Tool call sequences                     |        |
|  |   - Text responses  - Error handling                          |        |
|  |   - Plans generated - Resource usage                          |        |
|  +---------------------------------------------------------------+        |
|                              |                                             |
|  STAGE 2: JUDGE ASSIGNMENT                                                 |
|  +---------------------------------------------------------------+        |
|  | Map dimensions to judges:                                     |        |
|  |   Code Quality    --> qwen2.5-coder (GPU, fast)               |        |
|  |   Logical Accuracy --> hermes-3 (CPU, thorough)               |        |
|  |   Completeness    --> dolphin-3 (CPU, broad)                  |        |
|  |   Visual Output   --> llava (GPU, if applicable)              |        |
|  |   Complex Tasks   --> Claude API (cloud, high-stakes)         |        |
|  +---------------------------------------------------------------+        |
|                              |                                             |
|  STAGE 3: PARALLEL EVALUATION                                              |
|  +---------------------------------------------------------------+        |
|  |  [hermes-3]    [qwen-coder]    [dolphin-3]    [claude?]       |        |
|  |     |              |               |              |           |        |
|  |   Score A        Score B         Score C        Score D       |        |
|  |   + Reasoning    + Reasoning     + Reasoning    + Reasoning   |        |
|  +---------------------------------------------------------------+        |
|                              |                                             |
|  STAGE 4: CONSENSUS & DEBATE                                               |
|  +---------------------------------------------------------------+        |
|  | IF max(scores) - min(scores) > 0.2:                           |        |
|  |   --> Initiate structured debate (max 3 rounds)               |        |
|  |   --> Each judge sees others' reasoning                       |        |
|  |   --> Re-score after deliberation                             |        |
|  | ELSE:                                                         |        |
|  |   --> Accept weighted average                                 |        |
|  +---------------------------------------------------------------+        |
|                              |                                             |
|  STAGE 5: CALIBRATION & OUTPUT                                             |
|  +---------------------------------------------------------------+        |
|  | Compare against golden dataset (every N evaluations)          |        |
|  | Apply bias corrections (position, verbosity, leniency)        |        |
|  | Generate final report with confidence intervals               |        |
|  | Store to PostgreSQL (evaluation_results table)                |        |
|  +---------------------------------------------------------------+        |
|                                                                            |
|  OUTPUT: Structured evaluation report with:                                |
|    - Overall score (0.0-1.0) + confidence                                  |
|    - Per-dimension scores + reasoning                                      |
|    - Identified weaknesses + improvement suggestions                       |
|    - Historical trend comparison                                           |
+===========================================================================+
```

### 5.2 Debate Protocol

```python
async def multi_agent_debate(
    scores: dict[str, dict],  # judge_id -> {score, reasoning}
    context: str,
    max_rounds: int = 3,
    convergence_threshold: float = 0.15
) -> dict:
    """Structured debate between judge agents when scores diverge."""

    for round_num in range(max_rounds):
        # Check convergence
        score_values = [s["score"] for s in scores.values()]
        if max(score_values) - min(score_values) <= convergence_threshold:
            break

        # Each judge sees all other judges' reasoning
        for judge_id, judge_data in scores.items():
            other_reasoning = "\n".join([
                f"Judge {other_id} scored {other_data['score']}/5: {other_data['reasoning']}"
                for other_id, other_data in scores.items()
                if other_id != judge_id
            ])

            debate_prompt = f"""Round {round_num + 1} of evaluation debate.

Context: {context}

Your previous score: {judge_data['score']}/5
Your reasoning: {judge_data['reasoning']}

Other judges' assessments:
{other_reasoning}

Consider the other perspectives. You may adjust your score if
you find their reasoning compelling, or defend your position
with additional justification.

REASONING: <your updated reasoning>
SCORE: <your updated score 1-5>"""

            updated = await evaluate_with_ollama(
                model=JUDGE_MODELS[judge_id]["model"],
                prompt=debate_prompt,
                system=JUDGE_MODELS[judge_id]["system_prompt"]
            )
            scores[judge_id] = updated

    # Final aggregation
    final_scores = [s["score"] for s in scores.values()]
    return {
        "consensus_score": sum(final_scores) / len(final_scores),
        "score_range": max(final_scores) - min(final_scores),
        "rounds_needed": round_num + 1,
        "individual_scores": scores,
        "converged": (max(final_scores) - min(final_scores)) <= convergence_threshold
    }
```

---

## 6. Metrics and Scoring Rubrics

### 6.1 Universal Scoring Rubric (5-Point Scale)

```
SCORE 5 - EXCELLENT
  The output fully satisfies all requirements with high quality.
  No errors, complete coverage, efficient execution.
  Would be accepted without revision in a professional context.

SCORE 4 - GOOD
  The output satisfies most requirements with good quality.
  Minor issues that do not affect core functionality.
  Would require only minor revisions.

SCORE 3 - ADEQUATE
  The output partially satisfies requirements.
  Some notable issues or missing elements.
  Requires moderate revision to meet standards.

SCORE 2 - POOR
  The output fails to satisfy key requirements.
  Significant errors or omissions present.
  Requires substantial revision or rework.

SCORE 1 - UNACCEPTABLE
  The output does not meaningfully address the task.
  Critical errors, completely missing requirements.
  Must be redone from scratch.
```

### 6.2 Dimension-Specific Rubrics

**Code Quality Rubric:**

| Dimension | Weight | 5 (Excellent) | 3 (Adequate) | 1 (Unacceptable) |
|-----------|--------|---------------|---------------|-------------------|
| Correctness | 0.30 | Compiles, passes all tests, handles edge cases | Compiles, passes basic tests, some edge cases missed | Does not compile or fails basic tests |
| Security | 0.20 | No vulnerabilities, proper input validation | Minor security concerns, basic validation | SQL injection, XSS, or other critical vulnerabilities |
| Efficiency | 0.15 | Optimal time/space complexity | Acceptable performance, some optimization possible | O(n^2+) where O(n) possible, memory leaks |
| Readability | 0.15 | Clear naming, documented, idiomatic | Mostly readable, some unclear sections | Obfuscated, no comments, non-idiomatic |
| Completeness | 0.20 | All requirements implemented with tests | Core requirements met, missing some features | Major features missing |

**Task Completion Rubric:**

| Dimension | Weight | 5 (Excellent) | 3 (Adequate) | 1 (Unacceptable) |
|-----------|--------|---------------|---------------|-------------------|
| Goal Achievement | 0.35 | All goals fully achieved | Primary goals met, secondary incomplete | Primary goals not met |
| Tool Usage | 0.20 | Correct tools, minimal calls, proper arguments | Mostly correct, some unnecessary calls | Wrong tools, incorrect arguments |
| Reasoning | 0.20 | Clear logical chain, good planning | Acceptable reasoning with some gaps | No clear reasoning, chaotic execution |
| Error Handling | 0.15 | Graceful recovery, informative errors | Basic error handling | Crashes on errors, no recovery |
| Efficiency | 0.10 | Minimal steps, no redundancy | Some redundant steps | Excessive steps, circular reasoning |

### 6.3 Automated Rubric Generation (MAJ-EVAL Pattern)

```python
async def generate_rubric_from_documents(
    domain_docs: list[str],
    task_type: str,
    model: str = "hermes-3:8b"
) -> dict:
    """Generate evaluation rubric from domain documents (MAJ-EVAL pattern)."""

    prompt = f"""Analyze these domain documents and extract evaluation dimensions
for {task_type} tasks:

Documents:
{chr(10).join(domain_docs[:3])}  # Limit to avoid context overflow

For each dimension, provide:
1. Dimension name
2. Description (what it measures)
3. Weight (0.0-1.0, must sum to 1.0)
4. Scoring criteria for scores 1, 3, and 5

Format as JSON:
{{
  "dimensions": [
    {{
      "name": "...",
      "description": "...",
      "weight": 0.0,
      "criteria": {{
        "1": "...",
        "3": "...",
        "5": "..."
      }}
    }}
  ]
}}"""

    result = await evaluate_with_ollama(model=model, prompt=prompt,
        system="You are an evaluation rubric designer. Create precise, measurable criteria.")
    return json.loads(result["raw_text"])
```

---

## 7. Claude Code Hooks Integration

### 7.1 Hook Events for Agent Evaluation

Claude Code provides these hook events relevant to evaluation:

| Event | Trigger | Evaluation Use |
|-------|---------|---------------|
| `PreToolUse` | Before any tool execution | Validate tool selection against plan |
| `PostToolUse` | After tool completion | Evaluate tool output quality |
| `Stop` | Agent finishes response | Run full evaluation pipeline |
| `SessionStart` | New session begins | Load evaluation config, golden dataset |
| `UserPromptSubmit` | User sends message | Classify task complexity for judge selection |
| `Notification` | Agent sends alert | Monitor for error patterns |

### 7.2 Evaluation Hook Implementations

**PostToolUse -- Real-time Tool Quality Check:**
```bash
#!/bin/bash
# hooks/eval-post-tool.sh
# Runs after every tool use to check quality

TOOL_NAME="$1"
TOOL_OUTPUT="$2"

# Quick heuristic checks (fast, no LLM needed)
case "$TOOL_NAME" in
  "Write"|"Edit")
    # Check for common code quality issues
    if echo "$TOOL_OUTPUT" | grep -qE "TODO|FIXME|HACK|XXX"; then
      echo "WARNING: Output contains unresolved markers (TODO/FIXME/HACK)"
    fi
    ;;
  "Bash")
    # Check for dangerous commands
    if echo "$TOOL_OUTPUT" | grep -qE "rm -rf|sudo rm|DROP TABLE"; then
      echo "BLOCKED: Dangerous command detected"
      exit 1
    fi
    ;;
esac
```

**Stop -- Full Evaluation Pipeline:**
```python
#!/usr/bin/env python3
"""hooks/eval-on-stop.py -- Run full evaluation when agent completes."""

import json
import sys
import httpx

def evaluate_session(session_data: dict):
    """Trigger multi-agent evaluation of completed session."""

    # Extract trace from session
    trace = {
        "tool_calls": session_data.get("tool_calls", []),
        "llm_responses": session_data.get("responses", []),
        "errors": session_data.get("errors", []),
        "duration": session_data.get("duration_ms", 0)
    }

    # Send to local evaluation endpoint
    response = httpx.post(
        "http://localhost:8020/evaluate",  # Nexus evaluation service
        json={
            "trace": trace,
            "rubric": "default",
            "judges": ["accuracy", "code_quality", "completeness"],
            "debate_enabled": True
        },
        timeout=120.0
    )

    result = response.json()

    # Log results
    print(f"Evaluation Score: {result['consensus_score']:.2f}")
    print(f"Confidence: {result['confidence']:.2f}")
    for dim, score in result.get("dimensions", {}).items():
        print(f"  {dim}: {score:.2f}")

    # Flag for human review if low confidence
    if result["confidence"] < 0.5:
        print("FLAG: Low confidence -- human review recommended")

    return result

if __name__ == "__main__":
    data = json.loads(sys.stdin.read())
    evaluate_session(data)
```

### 7.3 Claude Code settings.json Hook Configuration

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "command": "/opt/ork-station/Nexus/hooks/eval-post-tool.sh \"$TOOL_NAME\" \"$TOOL_OUTPUT\""
      }
    ],
    "Stop": [
      {
        "command": "python3 /opt/ork-station/Nexus/hooks/eval-on-stop.py"
      }
    ],
    "SessionStart": [
      {
        "command": "python3 /opt/ork-station/Nexus/hooks/eval-session-init.py"
      }
    ]
  }
}
```

---

## 8. Implementation Recommendations for Nexus

### 8.1 Phased Implementation Plan

**Phase 1: Foundation (Week 1-2)**
- Implement single-judge evaluation with Ollama (hermes-3)
- Build trace capture system in Rust/Tauri backend
- Create basic scoring rubric engine
- Store results in PostgreSQL

**Phase 2: Multi-Judge (Week 3-4)**
- Add judge panel with role assignment (accuracy, code, completeness)
- Implement parallel evaluation via Tokio async
- Build debate protocol for divergent scores
- Add golden dataset calibration

**Phase 3: Intelligence (Week 5-6)**
- Implement G-Eval adaptation for local models
- Add automated rubric generation (MAJ-EVAL pattern)
- Build trend analysis and drift detection
- Create evaluation dashboard in frontend

**Phase 4: Integration (Week 7-8)**
- Claude Code hooks for real-time evaluation
- Prompt optimization loop (Build-Judge-Optimize)
- Human review interface for flagged cases
- Export/import evaluation profiles

### 8.2 Rust/Tauri Backend Structure

```
nexus/src-tauri/src/
  evaluation/
    mod.rs                  # Module root
    orchestrator.rs         # Evaluation pipeline orchestrator
    judge.rs               # Judge trait + implementations
    ollama_judge.rs        # Ollama-backed judge
    cloud_judge.rs         # Claude API judge (fallback)
    rubric.rs              # Rubric engine (load, generate, validate)
    debate.rs              # Multi-agent debate protocol
    calibration.rs         # Golden dataset calibration
    metrics.rs             # Score aggregation + confidence intervals
    trace.rs               # Agent trace capture + parsing
    storage.rs             # PostgreSQL evaluation results
```

### 8.3 Key Tauri Commands

```rust
// Evaluation Tauri commands for frontend integration

#[tauri::command]
async fn evaluate_agent_output(
    trace: AgentTrace,
    rubric_id: String,
    judges: Vec<String>,
    debate_enabled: bool,
) -> Result<EvaluationResult, String> { ... }

#[tauri::command]
async fn get_evaluation_history(
    agent_id: String,
    limit: usize,
) -> Result<Vec<EvaluationResult>, String> { ... }

#[tauri::command]
async fn create_rubric(
    name: String,
    dimensions: Vec<RubricDimension>,
) -> Result<String, String> { ... }

#[tauri::command]
async fn run_calibration(
    golden_set_id: String,
) -> Result<CalibrationReport, String> { ... }

#[tauri::command]
async fn get_evaluation_trends(
    agent_id: String,
    days: usize,
) -> Result<TrendReport, String> { ... }
```

### 8.4 Database Schema

```sql
-- Nexus Agent Evaluation Schema

CREATE TABLE nexus.evaluation_rubrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT,
    dimensions JSONB NOT NULL,  -- [{name, weight, criteria}]
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE nexus.evaluation_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agent_id TEXT NOT NULL,
    task_id TEXT,
    rubric_id UUID REFERENCES nexus.evaluation_rubrics(id),
    overall_score FLOAT NOT NULL,
    confidence FLOAT NOT NULL,
    dimension_scores JSONB NOT NULL,  -- {dimension: {score, reasoning}}
    judge_scores JSONB NOT NULL,      -- {judge_id: {score, reasoning, model}}
    debate_log JSONB,                 -- [{round, scores, reasoning}]
    trace_summary JSONB,              -- Compressed agent trace
    flagged_for_review BOOLEAN DEFAULT FALSE,
    human_score FLOAT,                -- Human override score (if reviewed)
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE nexus.golden_datasets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    samples JSONB NOT NULL,  -- [{input, output, human_score, dimensions}]
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE nexus.calibration_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    golden_set_id UUID REFERENCES nexus.golden_datasets(id),
    agreement_score FLOAT NOT NULL,    -- Judge vs human agreement
    bias_report JSONB,                 -- {position_bias, verbosity_bias, ...}
    drift_detected BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for fast queries
CREATE INDEX idx_eval_results_agent ON nexus.evaluation_results(agent_id, created_at DESC);
CREATE INDEX idx_eval_results_flagged ON nexus.evaluation_results(flagged_for_review) WHERE flagged_for_review = TRUE;
```

### 8.5 Offline-First Evaluation Strategy

Since Nexus follows the ORK-Station offline-first architecture:

```
EVALUATION PRIORITY ORDER:
  1. Local Ollama judges (hermes-3, qwen-coder, dolphin-3)
     --> Free, fast, private, no internet required
     --> Use for 90% of evaluations

  2. Prometheus-2 (7B, if available on GPU)
     --> Purpose-built evaluator, excellent accuracy
     --> Use for high-stakes evaluations

  3. Claude API (cloud fallback)
     --> Only for complex reasoning tasks
     --> Only when local judges disagree significantly
     --> Requires explicit user opt-in

  4. Human review (UI panel)
     --> For flagged cases (low confidence < 0.5)
     --> For golden dataset maintenance
     --> For calibration verification
```

---

## 9. Morpheus Tutorials Reference

### "KI am Murksen hindern" Concept

The search for the specific video "KI am Murksen hindern" (literally "preventing AI from messing around/bungling") by Morpheus Tutorials (Cedric Moessner) did not return direct results. The video may be:
- A recent upload not yet indexed
- Using a slightly different title
- Part of a longer tutorial series

**The Morpheus Tutorials channel** (by Cedric Moessner, based in Germany) covers:
- AI and machine learning tutorials in German
- Cybersecurity and IT education
- Python, Java, web development
- Over 2000 videos since 2011
- Master's degree in CS with ML and IT security specializations

**Conceptual Relevance to This Research:**

The concept "KI am Murksen hindern" (stopping AI from bungling) directly aligns with the Agent-as-a-Judge paradigm:

| Concept | Implementation |
|---------|---------------|
| Detect AI errors | Multi-agent evaluation catches mistakes |
| Prevent bad outputs | PreToolUse hooks block dangerous actions |
| Quality gates | PostToolUse evaluation scores each step |
| Continuous improvement | Build-Judge-Optimize feedback loop |
| Human oversight | Flagged review for low-confidence outputs |

The core idea -- using structured evaluation to prevent AI from producing low-quality or incorrect outputs -- is exactly what Agent-as-a-Judge solves at scale.

---

## 10. Sources

### Papers

1. Zhuge et al., "Agent-as-a-Judge: Evaluate Agents with Agents" -- https://arxiv.org/abs/2410.10934
2. You et al., "A Survey on Agent-as-a-Judge" -- https://arxiv.org/abs/2601.05111
3. Chen et al., "Multi-Agent-as-Judge" -- https://arxiv.org/abs/2507.21028
4. Yu, "When AIs Judge AIs" -- https://arxiv.org/abs/2508.02994
5. Breen Herrera et al., "Build, Judge, Optimize" -- https://arxiv.org/abs/2603.03565
6. Liu et al., "G-Eval: NLG Evaluation using GPT-4 with Better Human Alignment" -- https://arxiv.org/abs/2303.16634
7. Kim et al., "Prometheus: Inducing Fine-grained Evaluation Capability" -- https://arxiv.org/abs/2310.08491
8. Kim et al., "Prometheus 2" -- https://arxiv.org/abs/2405.01535

### Frameworks and Tools

9. DeepEval Agent Evaluation Guide -- https://deepeval.com/guides/guides-ai-agent-evaluation
10. DeepEval Agent Evaluation Metrics -- https://deepeval.com/guides/guides-ai-agent-evaluation-metrics
11. DeepEval GitHub -- https://github.com/confident-ai/deepeval
12. Prometheus-Eval GitHub -- https://github.com/prometheus-eval/prometheus-eval
13. G-Eval (DeepEval Implementation) -- https://deepeval.com/docs/metrics-llm-evals

### Best Practices and Guides

14. "LLM-as-a-Judge: 7 Best Practices" (Monte Carlo Data) -- https://www.montecarlodata.com/blog-llm-as-judge/
15. "LLM-as-a-Judge Complete Guide" (Evidently AI) -- https://www.evidentlyai.com/llm-guide/llm-as-a-judge
16. "Using LLMs for Evaluation" (Cameron Wolfe) -- https://cameronrwolfe.substack.com/p/llm-as-a-judge
17. "Justice or Prejudice? Quantifying Biases in LLM-as-a-Judge" -- https://llm-judge-bias.github.io/
18. "LLM-as-a-Judge Evaluation" (Langfuse) -- https://langfuse.com/docs/evaluation/evaluation-methods/llm-as-a-judge

### Local/Offline Evaluation

19. "You Can Now Use Ollama for LLM-as-a-Judge" -- https://medium.com/@jeffreyip54/you-can-now-use-ollama-for-llm-as-a-judge-76f06e3005c9
20. "Local LLM-as-judge with Prometheus and llamafile" (Mozilla AI) -- https://blog.mozilla.ai/local-llm-as-judge-evaluation-with-lm-buddy-prometheus-and-llamafile/
21. "Model picking with Fireworks Eval Protocol + Ollama" -- https://fireworks.ai/blog/llm-judge-eval-protocol-ollama

### Claude Code Hooks

22. "Automate workflows with hooks" (Claude Code Docs) -- https://code.claude.com/docs/en/hooks-guide
23. "Claude Code Hooks: All 12 Lifecycle Events" (Pixelmojo) -- https://www.pixelmojo.io/blogs/claude-code-hooks-production-quality-ci-cd-patterns
24. "Claude Code Hooks Mastery" (GitHub) -- https://github.com/disler/claude-code-hooks-mastery
25. "Claude Code: Hooks for Automated Quality Checks" (Letanure) -- https://www.letanure.dev/blog/2025-08-06--claude-code-part-8-hooks-automated-quality-checks

### Channel Reference

26. The Morpheus Tutorials -- https://www.the-morpheus.de/
27. TheMorpheus407 GitHub -- https://github.com/TheMorpheus407
