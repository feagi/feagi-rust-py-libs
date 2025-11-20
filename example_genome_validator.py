#!/usr/bin/env python3
"""
Example of using the Rust-based genome validator from Python.

This demonstrates how nrs-composer can validate genomes using the
same source of truth as feagi-core, with zero code duplication.
"""

import json
from feagi_rust_py_libs.genome import validate_genome, auto_fix_genome, PyValidationResult

# Example genome with issues
genome_with_issues = {
    "version": "2.0",
    "genome_id": "g-test123",
    "genome_title": "Test Genome",
    "genome_description": "Testing validation",
    "blueprint": {},
    "neuron_morphologies": {},
    "physiology": {
        "simulation_timestep": 0.01,
        "max_age": 100,
        "quantization_precision": "fp32"
    },
    "brain_regions": {},
    "signatures": {
        "genome": "abc",
        "blueprint": "def",
        "physiology": "ghi"
    },
    "stats": {}
}

def main():
    print("=" * 60)
    print("FEAGI Genome Validator - PyO3 Rust Binding Example")
    print("=" * 60)
    
    # Convert genome to JSON string
    genome_json = json.dumps(genome_with_issues)
    
    # Validate genome
    print("\n1. Validating genome...")
    result: PyValidationResult = validate_genome(genome_json)
    
    print(f"\n   Valid: {result.valid}")
    print(f"   Errors: {len(result.errors)}")
    print(f"   Warnings: {len(result.warnings)}")
    
    if result.errors:
        print("\n   Errors:")
        for error in result.errors:
            print(f"      ❌ {error}")
    
    if result.warnings:
        print("\n   Warnings:")
        for warning in result.warnings:
            print(f"      ⚠️  {warning}")
    
    # Auto-fix common issues
    print("\n2. Auto-fixing common issues...")
    fixed_json, num_fixes = auto_fix_genome(genome_json)
    print(f"   Applied {num_fixes} automatic fixes")
    
    # Validate fixed genome
    print("\n3. Re-validating fixed genome...")
    result2 = validate_genome(fixed_json)
    print(f"   Valid: {result2.valid}")
    print(f"   Errors: {len(result2.errors)}")
    print(f"   Warnings: {len(result2.warnings)}")
    
    print("\n" + "=" * 60)
    print("✅ Validation complete!")
    print("=" * 60)

if __name__ == "__main__":
    main()

