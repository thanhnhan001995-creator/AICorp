# Organization Domain Contract — AI Corp

- **Contract ID:** `AIC-CON-ORG-20261106-01`
- **Created:** 2026-11-06 (Asia/Bangkok)
- **Task referenced:** [`Tasks.md`](../../Tasks.md) — `T-01.01`
- **Brief referenced:** [`brief.md`](../../brief.md) — `AIC-BRF-20261105-01`, mục 7
- **Ideas referenced:** [`ideas.md`](../../ideas.md) — `I-02`, `I-03`
- **Status:** accepted contract for the J-01 foundation; implementation-agnostic.

## 1. Purpose

This contract defines the organization language shared by every later AI Corp component. It deliberately does not choose a UI, database, provider, or runtime protocol. Implementations must preserve these concepts and invariants.

## 2. Stable identifiers

Every persisted business entity has a stable, opaque ID. An ID is never derived from a display name, role, provider, or array position; renaming or changing a runtime never changes the ID.

| Entity | ID prefix | Example |
| --- | --- | --- |
| Organization | `org_` | `org_01hzy8c5q0m9w2x7a4r6k3n8pf` |
| Employee | `emp_` | `emp_01hzy8e7v4b2m6q9x5c3j1t8kd` |
| Role | `role_` | `role_01hzy8g2r7n4p9v6s3d1f8k5cq` |
| Runtime binding | `rtb_` | `rtb_01hzy8j8m3q5w7x9c2v4n6p1ka` |
| Authority envelope | `auth_` | `auth_01hzy8m4d6r8t2y5u7i9o1p3as` |
| Work item | `work_` | `work_01hzy8p6f2g4h7j9k3l5z8x1cv` |

The concrete generator may be UUIDv7, ULID, or an equivalent collision-resistant generator. IDs must remain unique within an organization and be preserved on import/export.

## 3. Core entities

### 3.1 Organization

An organization is the top-level local-first workspace. It has exactly one current CEO reference, an employee roster, roles, authority envelopes and future work records.

```json
{
  "id": "org_01hzy8c5q0m9w2x7a4r6k3n8pf",
  "name": "AI Corp",
  "ceo": { "kind": "human", "displayName": "CEO" },
  "status": "active"
}
```

The CEO is an operator identity, not an AI employee. A future multi-user system may extend this field, but must not imply that an AI employee is the CEO without an explicit product decision.

### 3.2 Employee

An employee is a persistent managed AI actor. It owns a display identity and history, but its role, runtime and authority are separate references.

```json
{
  "id": "emp_01hzy8e7v4b2m6q9x5c3j1t8kd",
  "organizationId": "org_01hzy8c5q0m9w2x7a4r6k3n8pf",
  "displayName": "Lan Research",
  "roleId": "role_01hzy8g2r7n4p9v6s3d1f8k5cq",
  "managerEmployeeId": null,
  "runtimeBindingId": "rtb_01hzy8j8m3q5w7x9c2v4n6p1ka",
  "status": "idle"
}
```

Allowed employee status values in this foundation are `idle`, `assigned`, `working`, `waiting_review`, `blocked`, `paused`, and `inactive`. A status is operational state, not proof that work is accepted.

### 3.3 Role

A role describes expected responsibility, not the model/provider/tool used to execute it.

```json
{
  "id": "role_01hzy8g2r7n4p9v6s3d1f8k5cq",
  "organizationId": "org_01hzy8c5q0m9w2x7a4r6k3n8pf",
  "title": "Research Lead",
  "description": "Leads research work and submits evidence-based reports."
}
```

Changing an employee's role changes responsibility expectations; it does not silently grant new authority or replace a runtime.

### 3.4 Reporting line

`managerEmployeeId` defines the employee reporting line. It may be `null` for an employee reporting directly to the CEO.

Invariants:

- the manager must belong to the same organization;
- an employee cannot manage itself;
- the graph must be acyclic;
- the CEO is outside the employee graph and is the implicit manager of root employees;
- a future Branch Director remains an employee in this graph. Branches are not introduced by this contract.

### 3.5 Runtime binding

A runtime binding tells AI Corp how a specific employee is currently reached. It must not contain a secret value in a human-readable profile or exported contract.

```json
{
  "id": "rtb_01hzy8j8m3q5w7x9c2v4n6p1ka",
  "kind": "local_model",
  "displayName": "Local research runtime",
  "connectionReference": "local-runtime-profile-1",
  "status": "available"
}
```

Allowed `kind` values initially are `local_model`, `api_agent`, `coding_agent`, `script`, and `mcp_agent`. `connectionReference` points to a separately protected configuration; it is not an API key, token, command line or secret.

### 3.6 Authority envelope

An authority envelope is the CEO-issued, readable boundary for what an employee may do in a role or for a particular work item.

```json
{
  "id": "auth_01hzy8m4d6r8t2y5u7i9o1p3as",
  "organizationId": "org_01hzy8c5q0m9w2x7a4r6k3n8pf",
  "subject": { "employeeId": "emp_01hzy8e7v4b2m6q9x5c3j1t8kd" },
  "scope": {
    "workItemIds": ["work_01hzy8p6f2g4h7j9k3l5z8x1cv"],
    "resources": ["report", "local_workspace"],
    "actions": ["view", "propose", "execute"]
  },
  "limits": {
    "validFrom": "2026-11-06T00:00:00+07:00",
    "validUntil": "2026-11-07T00:00:00+07:00",
    "budget": { "unit": "tokens", "maximum": 50000 },
    "delegation": "none",
    "selfApproval": false
  },
  "issuedBy": { "kind": "human", "displayName": "CEO" },
  "status": "active"
}
```

Action meanings:

| Action | Meaning |
| --- | --- |
| `view` | Read an allowed resource. |
| `propose` | Produce a recommendation without changing an external resource. |
| `execute` | Perform an allowed action inside the resource scope. |
| `delegate` | Create a child work item within the envelope's stated delegation limit. |
| `self_approve` | Mark a permitted result as approved without waiting for the CEO. |

Delegation values are `none`, `propose_only`, or `within_limits`. `selfApproval: true` must be explicit and never inferred from a role, model, manager, local runtime, or API runtime.

## 4. Effective authority rules

1. A request must identify the employee, target work item, requested action and resource.
2. An action is allowed only if at least one active CEO-issued envelope grants that exact action/resource/work scope and every applicable limit is satisfied.
3. A work-specific envelope may narrow or add a scope only when it is explicitly CEO-issued. It never bypasses expiry, budget or resource restrictions present in its own limits.
4. If multiple active envelopes apply, the implementation must record which envelope justified the action. It must never infer permission from the broadest envelope without that reference.
5. Revoked, expired, depleted or inactive envelopes grant no action.
6. `delegate` and `self_approve` are separate permissions. Granting one never grants the other.
7. The source runtime (`local_model`, `api_agent`, etc.) changes no authority decision.

## 5. Valid example

The following is valid because the researcher identity is stable, the role is separate from its local runtime, and the only requested action is within an active envelope.

```json
{
  "employeeId": "emp_01hzy8e7v4b2m6q9x5c3j1t8kd",
  "workItemId": "work_01hzy8p6f2g4h7j9k3l5z8x1cv",
  "requestedAction": "execute",
  "resource": "report",
  "authorityEnvelopeId": "auth_01hzy8m4d6r8t2y5u7i9o1p3as"
}
```

## 6. Invalid examples

| Invalid case | Why it must be rejected |
| --- | --- |
| An employee ID is regenerated after its display name changes. | Breaks durable identity and history. |
| A role name such as `Research Lead` is used as the provider/model identifier. | Conflates responsibility with runtime. |
| Employee A sets `managerEmployeeId` to Employee B while B already reports, directly or indirectly, to A. | Creates a reporting cycle. |
| A local model receives `execute` permission because it is local. | Authority cannot be inferred from runtime origin. |
| `self_approve` is implied because `delegate` is true. | They are distinct CEO-granted permissions. |
| A binding stores `apiKey` in the employee profile or this contract. | Secrets must remain in protected configuration. |
| An expired envelope is accepted because the employee has the same role. | Expired authority grants no action. |

## 7. Verification checklist for T-01.01

- [ ] Every core entity has a prefix and stable-ID rule.
- [ ] Role, employee and runtime are separate objects/references.
- [ ] Reporting-line cycle and cross-organization rules are explicit.
- [ ] Authority lists action, resource, work scope, time, budget, delegation and self-approval.
- [ ] Local/API runtime origin is explicitly excluded from permission inference.
- [ ] Valid and invalid examples make the contract testable in T-01.05.

## 8. Handoff to T-01.02

T-01.02 must implement local persistence for the entity contracts in sections 2–3 without changing their invariants. The CEO approved Tauri + React/TypeScript + Rust + SQLite on 2026-11-06: SQLite persistence and authority checks belong to Rust/Tauri; React/TypeScript uses only typed Rust APIs.
