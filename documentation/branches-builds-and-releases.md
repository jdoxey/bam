# Branches, Builds, and Releases

This document describes the branching strategy, build system, and release process for the bam programming language.

## Overview

The bam project uses a **GitLab Flow with environment branches** approach, combining structured release management with continuous integration. This provides quality gates while maintaining development velocity.

## Branch Strategy

### Permanent Branches

- **`develop`** - Integration branch for features. Always deployable but may contain unreleased features.
- **`release/X.Y`** - Long-lived release branches containing production-ready code. Multiple release branches may be active simultaneously for different versions in the wild.

### Temporary Branches

- **`feature/*`** - Individual feature development branches created from `develop`
- **`hotfix/X.Y.*`** - Emergency fixes created from specific release branches for critical production issues

## Build Types and Versioning

We follow [Semantic Versioning 2.0.0](https://semver.org/) with the following pre-release identifiers:

### Alpha Builds (`X.Y.Z-alpha.N`)
- **Trigger**: Pull requests to `develop`
- **Purpose**: Feature validation and integration testing
- **Requirement**: Must pass before PR merge
- **Audience**: Developers and CI/CD systems
- **Example**: `0.2.0-alpha.1`, `0.2.0-alpha.2`

### Beta Builds (`X.Y.Z-beta.N`)
- **Trigger**: Commits to `develop` branch
- **Purpose**: Integration testing and early user feedback
- **Frequency**: Automatic on every commit
- **Audience**: Internal testing and brave early adopters
- **Example**: `0.2.0-beta.1`, `0.2.0-beta.2`

### Release Candidates (`X.Y.Z-rc.N`)
- **Trigger**: Manual builds from `release/X.Y` branches
- **Purpose**: Final validation before production release
- **Testing**: Comprehensive QA and user acceptance testing
- **Audience**: QA team and selected beta users
- **Example**: `0.2.0-rc.1`, `0.2.0-rc.2`

### Final Release (`X.Y.Z`)
- **Trigger**: Tag on `release/X.Y` branch after RC approval
- **Purpose**: Production-ready stable release
- **Distribution**: Public release with full documentation
- **Audience**: All end users
- **Example**: `0.2.0`, `1.0.0`

## Workflow Process

### Feature Development
```bash
# Create feature branch from develop
git checkout develop
git pull origin develop
git checkout -b feature/new-syntax

# Work on feature, commit changes
# Push and create PR to develop
```

### Pull Request Validation
1. **Automatic alpha build** created (`X.Y.Z-alpha.N`)
2. **Required checks** must pass:
   - Compilation succeeds
   - All tests pass
   - Code quality checks pass
3. **Manual review** by maintainers
4. **Merge** only after alpha build validation

### Beta Release (Continuous)
```bash
# Every commit to develop triggers:
# 1. Automatic beta build (X.Y.Z-beta.N)
# 2. Artifact generation
# 3. Optional deployment to staging environment
```

### Release Preparation
```bash
# When develop is ready for release
git checkout develop
git pull origin develop
git checkout -b release/0.2

# Optional: version bump commits, changelog updates
# Create release candidate builds for testing
```

### Release Candidate Testing
1. **Manual trigger** RC builds from release branch
2. **Comprehensive testing**:
   - Cross-platform compatibility
   - Performance benchmarks
   - Integration testing
   - User acceptance testing
3. **Bug fixes** committed to release branch and merged back to `develop`
4. **New RC** created after fixes

### Final Release
```bash
# After RC approval, tag the release branch
git checkout release/0.2
git tag v0.2.0
git push origin release/0.2 --tags

# Merge back to develop to keep it current
git checkout develop
git merge --no-ff release/0.2
git push origin develop

# Keep release branch for ongoing maintenance
# release/0.2 remains active for hotfixes and patch releases
```

## Build Automation

### GitHub Actions Workflows

#### PR Validation (`.github/workflows/pr.yml`)
- Triggers on pull requests to `develop`
- Creates alpha builds
- Runs full test suite
- Blocks merge if any checks fail

#### Beta Builds (`.github/workflows/beta.yml`)
- Triggers on commits to `develop`
- Creates beta builds
- Publishes artifacts for testing
- Updates staging environments

#### Release Builds (`.github/workflows/release.yml`)
- Manual trigger for RC builds from release branches
- Automatic trigger for final releases on version tags
- Cross-platform builds (Linux, macOS, Windows)
- Bundles LLD linker for standalone distribution

## Version Precedence

Semantic versioning precedence (lowest to highest):
```
1.0.0-alpha.1 < 1.0.0-alpha.2 < 1.0.0-beta.1 < 1.0.0-beta.2 < 1.0.0-rc.1 < 1.0.0
```

## Quality Gates

### Alpha (PR) Requirements
- [ ] Code compiles successfully
- [ ] All unit tests pass
- [ ] Code formatting and linting pass
- [ ] No critical security vulnerabilities
- [ ] Peer review approval

### Beta (Develop) Requirements
- [ ] Integration tests pass
- [ ] Basic smoke tests pass
- [ ] No known critical bugs
- [ ] Feature flags properly configured

### Release Candidate Requirements
- [ ] Full test suite passes
- [ ] Performance benchmarks meet standards
- [ ] Cross-platform compatibility verified
- [ ] Documentation updated
- [ ] Security review completed

### Final Release Requirements
- [ ] RC testing completed with sign-off
- [ ] Release notes prepared
- [ ] Distribution packages tested
- [ ] Rollback plan documented

## Emergency Hotfixes

For critical production issues affecting a specific release:

```bash
# Create hotfix from the affected release branch
git checkout release/0.1
git checkout -b hotfix/0.1.1-critical-security-fix

# Make minimal fix, test thoroughly
# Merge back to release branch and tag
git checkout release/0.1
git merge --no-ff hotfix/0.1.1-critical-security-fix
git tag v0.1.1
git push origin release/0.1 --tags

# Merge back to develop to include fix in future releases
git checkout develop
git merge --no-ff release/0.1
git push origin develop

# Clean up hotfix branch
git branch -d hotfix/0.1.1-critical-security-fix
```

### Multiple Active Releases

When maintaining multiple versions simultaneously:

```bash
# Example: Fix affects both 0.1.x and 0.2.x
# Fix in oldest affected release first
git checkout release/0.1
git checkout -b hotfix/0.1.2-shared-fix

# Apply fix, merge to release/0.1, tag v0.1.2
git checkout release/0.1
git merge --no-ff hotfix/0.1.2-shared-fix
git tag v0.1.2

# Cherry-pick or merge to newer release
git checkout release/0.2
git merge --no-ff release/0.1  # or cherry-pick specific commits
git tag v0.2.1

# Finally merge to develop
git checkout develop
git merge --no-ff release/0.2
```

## Benefits of This Approach

1. **Quality Assurance**: Multiple validation stages catch issues early
2. **Continuous Integration**: Automatic builds provide fast feedback
3. **Flexibility**: Can ship features independently or bundle releases
4. **Multiple Version Support**: Maintain several active releases simultaneously
5. **Targeted Hotfixes**: Apply critical fixes to specific versions without affecting others
6. **Visibility**: Clear release status for all stakeholders
7. **Rollback Safety**: Structured process enables quick issue resolution
8. **Professional Appearance**: Industry-standard workflow builds confidence

## References

- [Semantic Versioning 2.0.0](https://semver.org/)
- [GitLab Flow Documentation](https://about.gitlab.com/topics/version-control/what-is-gitlab-flow/)
- [GitHub Flow vs Git Flow Comparison](https://www.atlassian.com/git/tutorials/comparing-workflows)