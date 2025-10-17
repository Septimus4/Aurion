---
title: Graph Model
version: 0.1.0
date: 2025-10-14
---
# Graph Model

Aurion stores all data in a labelled property graph database such as Neo4j or SurrealDB.  The property graph model uses nodes with labels and key–value properties and relationships with types, directions and properties【780551429696953†L227-L250】.  This section defines the canonical schema used by the orchestrator and plugins.

## Node labels and properties

| Label | Key properties | Description |
|------|---------------|-------------|
| `Engagement` | `id`, `name`, `created_at`, `owner` | Top-level container for a pentest.  All other nodes are scoped under an engagement. |
| `Scope` | `id`, `type` (cidr/domain/url), `value`, `created_at` | Defines the authorised target set.  A scope is linked to an `Engagement` via `:HAS_SCOPE`. |
| `Host` | `id`, `ip`, `hostname`, `os`, `created_at`, `updated_at` | Represents a host within the scope. |
| `Service` | `id`, `protocol`, `port`, `product`, `version`, `created_at` | A network service running on a host. |
| `ToolRun` | `id`, `plugin_name`, `plugin_version`, `started_at`, `ended_at`, `cache_key`, `risk_class` | Represents an invocation of a plugin. |
| `Evidence` | `id`, `type` (screenshot/log/json), `summary`, `cas_key`, `created_at` | Encapsulates a blob stored in the CAS. |
| `Finding` | `id`, `title`, `severity`, `description`, `status` (open/closed), `created_at`, `updated_at` | Represents a security issue derived from evidence. |
| `Vulnerability` | `cve_id`, `cvss_score`, `summary` | Optional link to external vulnerability metadata (e.g., CVE). |
| `User` | `id`, `name`, `role` | Represents an operator or collaborator. |

Provenance fields (`created_at`, `updated_at`, `source_plugin`) exist on all nodes to trace who created or updated data.  Plugins should populate `source_plugin` with their own name and version.

## Relationships

| Type | Direction | From → To | Description |
|-----|-----------|----------|-------------|
| `:HAS_SCOPE` | Engagement → Scope | An engagement defines a scope. |
| `:HAS_HOST` | Scope → Host | A host belongs to a scope. |
| `:HAS_SERVICE` | Host → Service | A service runs on a host. |
| `:HAS_TOOLRUN` | Service/Host → ToolRun | A tool run is associated with a host or service. |
| `:GENERATED` | ToolRun → Evidence | The tool run produced evidence. |
| `:HAS_FINDING` | Evidence → Finding | Evidence supports a finding. |
| `:RELATES_TO` | Finding ↔ Vulnerability | A finding relates to a known vulnerability. |
| `:ASSIGNED_TO` | Finding → User | Who is responsible for remediating a finding. |
| `:CREATED_BY` | (any) → User | Records authorship. |

All relationships carry provenance properties: `created_at`, `source_plugin` and optional context such as parameters used.

## Example upsert transaction

Plugins should upsert nodes to avoid duplication.  The following Cypher snippet creates or matches a host, service, tool run and evidence record in one transaction:

```cypher
MATCH (e:Engagement {id: $engagement_id})
MERGE (s:Scope {id: $scope_id})-[:HAS_SCOPE]->(e)
MERGE (h:Host {ip: $ip})-[:HAS_HOST]->(s)
MERGE (svc:Service {protocol: $protocol, port: $port})-[:HAS_SERVICE]->(h)
CREATE (tr:ToolRun {id: $run_id, plugin_name: $plugin, plugin_version: $version, started_at: timestamp(), cache_key: $cache})
CREATE (tr)-[:GENERATED]->(ev:Evidence {id: $evidence_id, type: $type, summary: $summary, cas_key: $cas_key, created_at: timestamp()})
CREATE (h)-[:HAS_TOOLRUN]->(tr)
```

This pattern avoids re-creating hosts or services when they already exist while always recording new `ToolRun` and `Evidence` nodes.  The orchestrator wraps all plugin mutations in transactions to ensure atomicity.

## Query examples

**List unresolved findings with evidence links**

```cypher
MATCH (f:Finding {status: 'open'})<-[:HAS_FINDING]-(ev:Evidence)
OPTIONAL MATCH (f)-[:RELATES_TO]->(v:Vulnerability)
RETURN f.id AS finding_id,
       f.title AS title,
       f.severity AS severity,
       v.cve_id AS cve,
       collect(ev.cas_key) AS evidence_cas_keys
ORDER BY f.severity DESC;
```

**Retrieve all services and their last tool run**

```cypher
MATCH (h:Host)-[:HAS_SERVICE]->(svc:Service)
OPTIONAL MATCH (svc)-[:HAS_TOOLRUN]->(tr:ToolRun)
WITH svc, max(tr.started_at) AS last_run
RETURN svc.protocol, svc.port, last_run;
```

## Acceptance Criteria

This graph model document defines node labels, properties and relationships for the Aurion graph schema.  It provides example upsert Cypher transactions and queries to list unresolved findings and recent tool runs.  Provenance fields and relationship semantics are explained.  At least one citation referencing the property graph model is included【780551429696953†L227-L250】.