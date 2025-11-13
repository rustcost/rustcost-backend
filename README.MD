# RustCost Backend

This repository contains the **backend** of [RustCost](https://github.com/rustcost/rustcost-helmchart)
, a lightweight Kubernetes-native cost monitoring and observability platform.
The backend is built with Rust (Axum, Diesel) and provides the API and metrics collection for the RustCost frontend.

---

### Developer Notes

These are for maintainers (not required for normal users
build

Version Guideline
Development phase → multiple -dev.N builds
Pre-release phase → one or more -rc.N builds
Final production release → 1.0.0 on December 21

```bash
docker build --build-arg APP_NAME=rustcost-core -t rustcost-core .
docker tag rustcost-core kimc1992/rustcost-core:1.0.0-dev.2
docker push kimc1992/rustcost-core:1.0.0-dev.2
```