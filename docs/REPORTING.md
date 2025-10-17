---
title: Reporting Model
version: 0.1.0
date: 2025-10-14
---
# Reporting

Aurion organises findings and evidence in the graph, but ultimately produces reports for clients and management.  Reports are generated from a structured **Finding model** and rendered with [Tera](https://tera.netlify.app/) templates.  This document defines the finding schema, describes evidence linking, provides template fragments and an example report excerpt.

## Finding model

Each `Finding` node in the graph has the following properties:

- `id` (string, unique within the engagement)
- `title` (short description)
- `severity` (info/low/medium/high/critical)
- `description` (detailed narrative)
- `status` (open, accepted, false_positive, fixed, closed)
- `tool_runs` (list of associated `ToolRun` IDs)
- `evidence_ids` (list of evidence CAS keys)
- `recommendation` (remediation guidance)
- `created_at`, `updated_at` (timestamps)

Findings may link to `Vulnerability` nodes via `:RELATES_TO` edges when they correspond to public CVEs.

## Evidence linking

Evidence is stored as blobs in the CAS.  Reports embed small evidence directly (e.g., HTTP response snippets) and link to larger artefacts via a stable hyperlink containing the CAS key.  The link scheme is:

```
cas://<cas_key>
```

The report generator resolves these links to download the artifact from the CAS when producing HTML or PDF outputs.  When generating Markdown only, the CAS link is left intact.

## Tera templates

Tera templates are used to convert the structured `Report` and `Finding` objects into Markdown or HTML.  Below is a fragment of the executive summary section:

```tera
{% macro severity_label(severity) %}
  {% if severity == "critical" %}🔥 Critical{% elif severity == "high" %}⚠️ High{% elif severity == "medium" %}⚠️ Medium{% elif severity == "low" %}🔎 Low{% else %}ℹ️ Info{% endif %}
{% endmacro %}

## Executive Summary

Total findings: {{ findings | length }}

| ID | Title | Severity | Status |
|----|-------|----------|--------|
{% for f in findings %}
| {{ f.id }} | {{ f.title }} | {{ severity_label(f.severity) }} | {{ f.status }} |
{% endfor %}
```

Another template section renders detailed finding entries:

```tera
{% for f in findings %}
### {{ f.id }} – {{ f.title }}
Severity: {{ f.severity }}

{{ f.description }}

**Evidence**
{% for e in f.evidence %}
- [{{ e.summary }}]({{ e.cas_link }})
{% endfor %}

**Recommendation**

{{ f.recommendation }}
{% endfor %}
```

## Example report excerpt

```markdown
## Executive Summary

Total findings: 2

| ID | Title | Severity | Status |
|----|-------|----------|--------|
| F-001 | Outdated Apache server | ⚠️ High | open |
| F-002 | Directory listing enabled | 🔎 Low | open |

### F-001 – Outdated Apache server

Severity: high

The Apache HTTP server running on `192.0.2.10:80` is version `2.4.38`.  Multiple vulnerabilities affect this version.

**Evidence**

- [Service version output](cas://abc123def456)

**Recommendation**

Upgrade Apache to at least version `2.4.58` and apply security updates.  Consider using an automated configuration management system to maintain patch levels.
```

## Acceptance Criteria

This reporting document defines a finding schema, explains evidence linking via CAS URIs, provides Tera template fragments for an executive summary and finding details, and includes an example report excerpt.  Evidence linking rules and severity labels are specified.  Acceptance criteria are clearly stated.