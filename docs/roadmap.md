# Naldom Project Roadmap

This document outlines the planned development phases and key milestones for the Naldom language.

```mermaid
gantt
    dateFormat  YYYY-MM-DD
    title Naldom Project Roadmap

    section Phase 1: Prototype (3 months)
    GitHub Setup & Init                  :done, 2025-08-15, 7d
    NLD Parser (LLM Integration)         :active, 2025-08-22, 30d
    Basic IntentGraph Definition         :2025-09-22, 15d
    Semantic Analyzer (Basic)            :2025-10-07, 15d
    IR-HL Generation                     :2025-10-22, 15d
    Python CodeGen & Runtime             :2025-11-06, 30d
    Tracing & Initial Unit Tests         :2025-12-06, 15d

    section Phase 2: Core Compiler & Multi-Target (6 months)
    IR-LL Development                    :2025-12-21, 45d
    LLVM IR Generation                   :2026-02-04, 45d
    WebAssembly Backend                  :2026-03-21, 30d
    Enhanced Semantic Analyzer           :2026-04-20, 30d
    Basic Optimizer                      :2026-05-20, 30d
    Security (Sandbox Init)              :2026-06-19, 15d
    Tests Expansion & IR-LL Verification :2026-07-04, 30d

    section Phase 3: Full Compiler & Ecosystem (12 months)
    Advanced Optimizer                   :2026-08-03, 60d
    Cross-Platform Runtime (Full)        :2026-10-02, 90d
    Full Target Code Generation          :2027-01-01, 60d
    IDE Plugins (VS Code)                :2027-03-02, 45d
    Developer Tools (REPL, Analyzer)     :2027-04-16, 45d
    Package Manager (naldom-pkg)         :2027-06-01, 30d
    Safety & Reverse Traceability        :2027-07-01, 30d
    Comprehensive End-to-End Testing     :2027-08-01, 30d
    ```