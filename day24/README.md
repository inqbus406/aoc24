# Digital Circuit Analysis - Finding Gate Swaps

## Problem
A combinational logic circuit (ripple-carry adder) has 4 pairs of gates that need to be swapped to work correctly. Find the 8 gates that need swapping.

## Approach

### 1. Understand Expected Structure
A proper N-bit ripple-carry adder follows these patterns:

- **Bit 0**: `z00 = x00 XOR y00`
- **Middle bits**: `z_i = (x_i XOR y_i) XOR carry_in`  
- **Final bit**: `z_N = carry_out` (OR gate)
- **Carry logic**: `carry_out = (x_i AND y_i) OR ((x_i XOR y_i) AND carry_in)`

### 2. Identify Suspicious Patterns

**Wrong gate types on outputs:**
- z outputs should be XOR (except final bit = OR)
- If `z11` is AND instead of XOR → suspicious

**Broken carry chain:**
- AND gates should feed OR gates (carry propagation)
- If `x15 AND y15` doesn't feed an OR → suspicious

**Missing connections:**
- Each `x_i XOR y_i` should be used in corresponding `z_i`
- If `z15` doesn't use `x15 XOR y15` → suspicious

### 3. Find Correct Swaps

For each suspicious z output:
1. Find what gate type it currently has
2. Find the gate that should be the z output
3. Identify the swap needed

**Example:**
- `z31 = x31 AND y31` (wrong - should be XOR)
- Find: `ctw = x31 XOR y31` 
- Find: `dmh = fgs XOR ctw` (uses the x31⊕y31 result)
- **Swap**: `z31 ↔ dmh`

### 4. Verification

Test the swaps by:
- Applying swaps to circuit
- Running test cases (small additions)
- Verify 100% success rate

## Tools Created

- `visualize.py` - Generate full circuit diagrams
- `find_suspicious.py` - Identify problematic gates  
- `analysis.py` - Systematic swap finder
- Generated PNGs for visual inspection

## Usage

**Generate visualizations:**
```bash
# Full circuit diagram
python3 visualize.py
dot -Tpng circuit.dot -o circuit.png

# Find suspicious gates and create focused view
python3 find_suspicious.py
dot -Tpng suspicious.dot -o suspicious.png

# Find the swaps needed
python3 analysis.py
```

**Prerequisites:**
- Install Graphviz: `brew install graphviz` (macOS) or `apt install graphviz` (Linux)

## Key Insight

Treat as **structural analysis** rather than simulation. Look for violations of expected mathematical relationships in binary addition, not just functional testing.
