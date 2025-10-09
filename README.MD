# RustCost Backend

This repository contains the **backend** of [RustCost](https://github.com/rustcost/rustcost-helmchart)
, a lightweight Kubernetes-native cost monitoring and observability platform.
The backend is built with Rust (Axum, Diesel) and provides the API and metrics collection for the RustCost frontend.

---

### Developer Notes

These are for maintainers (not required for normal users
build

```bash
docker build --build-arg APP_NAME=rustcost -t rustcost .
docker tag rustcost kimc1992/rustcost:latest
docker push kimc1992/rustcost:latest
```