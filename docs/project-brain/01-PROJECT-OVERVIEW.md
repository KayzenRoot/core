# CORE Project Overview

## Product
**CORE**

## Mission
CORE is a standalone, headless engineering runtime for governed execution, verification, recovery and delivery. Canonical project state comes from tracked Git sources and explicit GEF Work Orders/Context Locks.

## Current foundation
- M01 Runtime & Lifecycle: promoted
- M02 Project / Workspace Adapter: promoted, standalone V2 migration in progress
- M03 Work Order Engine: promoted, standalone V2 migration in progress
- M04: external previous-version compatibility decision remains gated
- M05/M06: planning only
- GEF v1.0.0 governance: installed

## Boundary
No local Docker project service, repository MCP server, external memory database or project-registration daemon is required to develop, test, review or boot CORE. Optional future external providers must use explicit provider-neutral contracts and cannot supersede canonical Git truth.
