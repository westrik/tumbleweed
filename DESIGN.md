### Introduction

_Tumbleweed_ is an opinionated framework for building isomorphic web applications in Rust.

Some features:

- Data types and API endpoints are configured via TOML
- Database migrations are handled automatically using diffs of the TOML schema (optional fallback to up.sql + down.sql)
- Test harnesses are automatically generated from entity and endpoint schemas
- Entity-relationship diagrams are automatically generated from entity schemas
