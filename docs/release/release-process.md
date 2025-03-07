### Tag a Release
- Decide on a version (e.g., v1.0.0).
- Create and push a tag:
```bash
git tag v1.0.0 # put release version
git push origin v1.0.0
```
This triggers the Release workflow because it matches the v* pattern.