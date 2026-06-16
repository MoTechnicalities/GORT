#!/usr/bin/env python3
"""
Schema evolution invariant validator.
Ensures new schema versions maintain backward compatibility or explicitly declare breaking changes.

Usage:
  validate-schema-evolution.py <manifest.json>

Exit codes:
  0: All versions satisfy evolution invariants
  1: Invariant violations detected (breaking change not declared, etc.)
  2: Invalid input
"""
import json
import sys
from pathlib import Path
from typing import Dict, List, Any, Optional, Set

def load_manifest(path: str) -> Optional[Dict[str, Any]]:
    """Load and parse manifest file."""
    try:
        with open(path) as f:
            return json.load(f)
    except (FileNotFoundError, json.JSONDecodeError) as e:
        print(f"error loading manifest: {e}", file=sys.stderr)
        return None

def load_schema(path: str) -> Optional[Dict[str, Any]]:
    """Load and parse schema file; return None if missing."""
    try:
        with open(path) as f:
            return json.load(f)
    except (FileNotFoundError, json.JSONDecodeError):
        return None

def extract_required_fields(schema: Dict[str, Any]) -> Set[str]:
    """Extract required fields from JSON schema."""
    return set(schema.get("required", []))

def extract_properties(schema: Dict[str, Any]) -> Dict[str, Any]:
    """Extract properties definitions from JSON schema."""
    return schema.get("properties", {})

def check_backward_compatibility(
    old_schema: Dict[str, Any],
    new_schema: Dict[str, Any],
    old_version: str,
    new_version: str
) -> tuple[bool, List[str]]:
    """
    Check if new_schema is backward compatible with old_schema.
    Return (is_compatible, issues).
    
    Backward compatibility means:
    1. All old required fields remain required in new schema
    2. All old properties retain their types (or become more permissive)
    3. No old properties are removed
    4. No NEW required fields are added
    """
    issues = []
    
    old_required = extract_required_fields(old_schema)
    new_required = extract_required_fields(new_schema)
    old_props = extract_properties(old_schema)
    new_props = extract_properties(new_schema)
    
    # Check 1: Old required fields still required in new
    lost_required = old_required - new_required
    if lost_required:
        issues.append(
            f"BREAKING: Required fields removed or demoted in {new_version}: {lost_required}"
        )
    
    # Check 2: Old properties not removed in new
    removed_props = set(old_props.keys()) - set(new_props.keys())
    if removed_props:
        issues.append(
            f"BREAKING: Properties removed in {new_version}: {removed_props}"
        )
    
    # Check 3: Type constraints tightened (e.g., string→integer)
    for prop_name in old_props:
        if prop_name not in new_props:
            continue  # Already caught above
        
        old_prop = old_props[prop_name]
        new_prop = new_props[prop_name]
        
        old_type = old_prop.get("type")
        new_type = new_prop.get("type")
        
        # If type changed to something more restrictive, that's breaking
        if old_type and new_type and old_type != new_type:
            issues.append(
                f"BREAKING: Type change for '{prop_name}': {old_type} → {new_type}"
            )
        
        # Check enum narrowing
        old_enum = set(old_prop.get("enum", []))
        new_enum = set(new_prop.get("enum", []))
        if old_enum and new_enum and old_enum > new_enum:
            removed_values = old_enum - new_enum
            issues.append(
                f"BREAKING: Enum values removed from '{prop_name}': {removed_values}"
            )
    
    # Check 4: New required fields added (old documents won't have them)
    new_required_fields = new_required - old_required
    if new_required_fields:
        issues.append(
            f"BREAKING: New required fields added in {new_version}: {new_required_fields}"
        )
    
    is_compatible = len(issues) == 0
    return is_compatible, issues

def validate_evolution_invariants(
    manifest: Dict[str, Any],
    schema_dir: Path
) -> tuple[bool, List[str]]:
    """
    Validate schema evolution invariants across all versions.
    
    Rules:
    1. If new version is backward compatible, no compatibility metadata needed
    2. If new version breaks compatibility, must explicitly declare it via:
       - `breaking_changes` list populated
       - `compatible_with` list only includes compatible versions
       - `requires_explicit_migration` set to true
    
    Return (all_valid, error_messages).
    """
    errors = []
    supported_versions = manifest.get("supported_versions", {})
    
    if not supported_versions:
        errors.append("Manifest has no supported_versions")
        return False, errors
    
    # Build version ordering (assume alphabetical or explicit ordering in manifest)
    versions = sorted(supported_versions.keys())
    if len(versions) < 2:
        # No evolution to check if only one version
        return True, []
    
    # Check each version against its predecessors
    for i, version in enumerate(versions):
        if i == 0:
            continue  # No predecessor
        
        prev_version = versions[i - 1]
        version_info = supported_versions[version]
        prev_info = supported_versions[prev_version]
        
        # Load schemas
        schema_path = schema_dir / version_info.get("schema_path", "")
        prev_schema_path = schema_dir / prev_info.get("schema_path", "")
        
        current_schema = load_schema(str(schema_path))
        prev_schema = load_schema(str(prev_schema_path))
        
        if not current_schema or not prev_schema:
            continue  # Skip if schemas unavailable
        
        # Check compatibility
        is_compatible, issues = check_backward_compatibility(
            prev_schema, current_schema, prev_version, version
        )
        
        if is_compatible:
            # Should NOT have breaking changes declared if actually compatible
            breaking_changes = version_info.get("breaking_changes", [])
            if breaking_changes:
                errors.append(
                    f"INVARIANT VIOLATION: {version} declares breaking changes "
                    f"but is actually backward compatible with {prev_version}"
                )
        else:
            # Has breaking changes - MUST be declared
            breaking_changes = version_info.get("breaking_changes", [])
            requires_migration = version_info.get("requires_explicit_migration", False)
            compatible_with = version_info.get("compatible_with", [])
            
            if not breaking_changes:
                errors.append(
                    f"INVARIANT VIOLATION: {version} has breaking changes vs {prev_version} "
                    f"but does not declare breaking_changes: {issues}"
                )
            
            if not requires_migration:
                errors.append(
                    f"INVARIANT VIOLATION: {version} has breaking changes but "
                    f"requires_explicit_migration is not true"
                )
            
            # Verify compatible_with excludes broken versions
            if prev_version in compatible_with and prev_version not in (prev_info.get("compatible_with", [])):
                # This is a new breaking point; prev_version should not be in compatible_with
                errors.append(
                    f"INVARIANT VIOLATION: {version} lists {prev_version} in compatible_with "
                    f"but has breaking changes vs it"
                )
    
    return len(errors) == 0, errors

def main():
    """Main entry point."""
    if len(sys.argv) < 2:
        print("Usage: validate-schema-evolution.py <manifest.json>", file=sys.stderr)
        sys.exit(2)
    
    manifest_path = sys.argv[1]
    manifest_dir = Path(manifest_path).parent
    
    manifest = load_manifest(manifest_path)
    if not manifest:
        sys.exit(2)
    
    all_valid, errors = validate_evolution_invariants(manifest, manifest_dir)
    
    if all_valid:
        print("✓ Schema evolution invariants satisfied")
        sys.exit(0)
    else:
        print("✗ Schema evolution invariant violations detected:")
        for error in errors:
            print(f"  {error}")
        sys.exit(1)

if __name__ == "__main__":
    main()
