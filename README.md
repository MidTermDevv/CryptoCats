# CryptoCats

<img width="2172" height="724" alt="ChatGPT Image Jul 13, 2026, 10_52_46 PM" src="https://github.com/user-attachments/assets/d8ef98e7-15a4-4dc0-af65-9017edb8c1d2" />

CryptoCats is a production-oriented Anchor workspace for a hybrid SPL-token and NFT experience built around a Pump.fun-launched $CATS token. The repository contains an on-chain claim program, a deterministic pixel-art renderer, a minimal wallet-facing frontend, deployment scripts, and a test scaffold for validating the core mint-and-claim flow.

X account: [@CTCatsfun](https://x.com/CTCatsfun)

The system is designed to be practical and extensible rather than purely academic. The on-chain program stores claim state in PDAs, records trait outcomes in claim receipts, and is structured so a future production deployment can swap in a verifiable randomness source such as Switchboard VRF and a real NFT metadata pipeline using Metaplex Token Metadata or the Core standard.

## What this repository includes

- An Anchor program in [programs/cryptocats](programs/cryptocats) implementing a claim lifecycle for NFT airdrops.
- A minimal Next.js frontend in [app](app) for viewing the collection concept and claim experience.
- Trait definitions and rarity weights in [traits/traits.json](traits/traits.json).
- A deterministic SVG renderer in [assets/render.py](assets/render.py) that converts trait IDs into pixel-art cat visuals.
- Deployment helpers in [scripts/deploy.sh](scripts/deploy.sh).
- Anchor tests in [tests/cryptocats.test.ts](tests/cryptocats.test.ts).

## System overview

<img width="1774" height="887" alt="ChatGPT Image Jul 13, 2026, 11_34_49 PM" src="https://github.com/user-attachments/assets/db3056a6-7c34-4160-8162-012fc9de9d92" />

CryptoCats follows a layered architecture:

1. Token layer
   - The project assumes an existing SPL token mint for $CATS is already deployed and configured externally.
   - The program is parameterized by that mint address and uses it as the eligibility anchor for claim events.
   - In a production deployment, the contract would be paired with an indexer or on-chain snapshot strategy to evaluate holder balance and holding duration.

2. Claim event layer
   - Claim events are represented as PDAs that define the claim mode, threshold, time window, and maximum number of claims.
   - The program supports three modes:
     - Mode 0: balance-based eligibility only.
     - Mode 1: time-windowed eligibility.
     - Mode 2: open or externally managed mode for future integration.
   - Each claim event is isolated by a unique PDA derived from the config pubkey and event id.

3. NFT minting layer
   - When a wallet qualifies, the program derives a deterministic trait set from a seed and a nonce.
   - The current implementation records those traits in a claim receipt and mints a single NFT token to the claimant.
   - The program is built so the minting step can be extended to include Metaplex metadata and on-chain attribute storage once the full NFT pipeline is connected.

4. Rendering and metadata layer
   - Trait IDs are compact and efficient for on-chain usage.
   - The renderer converts those IDs into deterministic SVG art, which is suitable for previews, web displays, and off-chain metadata generation.
   - The repository includes a trait configuration file that can be expanded into a full metadata JSON generator for Metaplex compatibility.

## Program structure

The Anchor program is implemented in [programs/cryptocats/src/lib.rs](programs/cryptocats/src/lib.rs). The key pieces are:

- Config account
  - Stores the authority, $CATS mint, threshold, claim mode, event duration, next event id, and next seed.
  - This acts as the root configuration state for the program.
  - It is the canonical place for general policy and defaults, and it is ideal for future upgrades because it centralizes global values.

- Claim event account
  - Represents a specific airdrop event and contains the eligibility rules and mint budget.
  - It also stores a seed used for deterministic trait derivation.
  - In a production system, this should be expanded to include a start/end timestamp, event-specific authority, and a stronger cap policy.

- Claim receipt account
  - Tracks whether a particular wallet has already claimed for a given event.
  - This is the replay-protection mechanism in the current scaffold.
  - The account is PDA-derived from the config, event, and claimant public key, which keeps the address deterministic and minimizes redundant lookups.

- Instruction flow
  - initialize_config: bootstraps the config.
  - create_claim_event: creates a timed or balance-based claim event.
  - claim_nft: verifies eligibility, derives traits, records the claim, and mints the NFT.

### Data model in practice

The current state model is intentionally compact:

```rust
pub struct Config {
    pub authority: Pubkey,
    pub cats_mint: Pubkey,
    pub threshold: u64,
    pub claim_mode: u8,
    pub event_duration: u64,
    pub next_event_id: u64,
    pub next_seed: u64,
    pub bump: u8,
}
```

```rust
pub struct ClaimReceipt {
    pub config: Pubkey,
    pub event: Pubkey,
    pub claimant: Pubkey,
    pub nft_mint: Pubkey,
    pub claimed_at_slot: u64,
    pub claimed: bool,
    pub traits: [u8; 5],
    pub bump: u8,
}
```

This keeps the account sizes predictable and makes the instruction logic easier to audit. In production, the same model can later be extended with richer event metadata, additional mint counters, or immutable rarity snapshots.

## Trait system and rarity model

Trait data is defined in [traits/traits.json](traits/traits.json). Each trait category contains multiple variants with a tier and a relative weight. The current scaffold uses a compact trait vector of five values:

- Fur
- Eyes
- Accessories
- Expression
- Background

The rendering logic in [assets/render.py](assets/render.py) uses those values to draw a deterministic pixel-art cat. The trait weights are intentionally simple and readable so they can be extended into a more formal rarity engine later.

The current derivation logic uses a deterministic seed and nonce to generate trait IDs. In a production deployment, this should be replaced by a verifiable randomness source such as Switchboard VRF. That protects the system from manipulation and makes the outcome cryptographically unpredictable before the claim is finalized.

## How the claim flow works

1. The authority initializes the program config and provides the $CATS mint address and claim policy.
2. The authority creates a claim event with defined eligibility parameters.
3. A user who holds enough $CATS and satisfies the event rules submits a claim.
4. The program checks the claim receipt to ensure the wallet has not already claimed that event.
5. The program derives a trait set deterministically.
6. The program records the event in the claim receipt and mints a single NFT to the claimant.

For production, the flow should be extended with:

- Balance snapshots or indexer-based holder tracking.
- Stronger anti-abuse controls.
- A verifiable randomness oracle.
- Metaplex metadata upload and attribute verification.

## Security and correctness considerations

This repository is intentionally structured to be understandable and modular. The current code prioritizes clarity and correctness over full production integration. The most important security points to keep in mind are:

- Authority controls should be restricted to the intended admin wallet or multisig.
- Claim events should be configured with strict thresholds and budgets.
- The claim receipt is the primary protection against replayed claims.
- The current randomness is deterministic and should not be treated as secure for mainnet.
- The token transfer path should be replaced with a formal balance-checking strategy that matches your actual indexing or snapshot approach.
- All admin actions should be gated and logged off-chain so that the authority can be monitored and rotated safely.
- Any production rollout should include a pause mechanism, upgrade policy, and explicit event cancellation rules.

### Why the current design is reasonably safe for a scaffold

The implementation uses PDAs for state and avoids unnecessary mutable data structures. It keeps the logic simple enough that each instruction can be reasoned about, audited, and expanded. The claim receipt ensures that the same wallet cannot repeatedly claim the same event, which is the core anti-replay feature in the current code.

### What must change before mainnet

Before deploying to mainnet, the following areas should be hardened:

1. Randomness
   - Replace deterministic seed generation with Switchboard VRF or another verifiable randomness oracle.

2. Authority model
   - Use a multisig or timelock-controlled authority rather than a single wallet.

3. Metadata pipeline
   - Upload metadata to Arweave, IPFS, or a similar storage layer and link NFT attributes to the claim event and trait payload.

4. Balance checks
   - Move eligibility from a simple account-based check to a trusted indexer or snapshot system that reflects true holder state.

## Frontend experience

The frontend in [app](app) is intentionally minimal. It provides a simple landing page and a placeholder collection experience for the user flow:

- It introduces the product concept.
- It explains that wallet connectivity and collection inspection would be added next.
- It keeps the UI lightweight so it can be expanded into a richer experience later.

A production frontend would add:

- Wallet connection support using Phantom, Solflare, Backpack, or Wallet Adapter.
- Wallet-based claim status and NFT collection views.
- Trait preview rendering directly in the browser.
- Event countdowns, eligibility messaging, and claim history.

## Deployment strategy

### Local development

1. Install Rust and Cargo.
2. Install Anchor and the Solana CLI.
3. Create a local keypair if needed.
4. Build the program:

```bash
anchor build
```

5. Run tests:

```bash
anchor test
```

6. Start the frontend:

```bash
cd app
npm install
npm run dev
```

### Example program configuration

The on-chain config is initialized from the authority and the $CATS mint address. A typical setup looks like this conceptually:

```rust
let authority = Pubkey::new_unique();
let cats_mint = Pubkey::new_unique();
let threshold = 100_000_000; // example amount in smallest units
let claim_mode = 1; // time-windowed event
let event_duration = 1000;
```

### Example trait rendering usage

The renderer can be executed directly from the repository root:

```bash
python3 assets/render.py
```

This outputs an SVG string for a deterministic pixel-art cat based on the trait vector.

### Example trait config excerpt

```json
{
  "fur": [
    { "id": 0, "name": "Tabby", "tier": "common", "weight": 45 }
  ],
  "eyes": [
    { "id": 1, "name": "Laser", "tier": "rare", "weight": 30 }
  ]
}
```

### Devnet

The deployment helper in [scripts/deploy.sh](scripts/deploy.sh) performs a build and deploy sequence for devnet. In a real deployment, you should update the authority, mint address, and environment variables before running it.

### Mainnet

Mainnet should only be attempted after:

- The randomness source is hardened.
- The claim policy and authority choreography are fully reviewed.
- The metadata pipeline is production-ready.
- The program has passed both unit and integration tests in a staging environment.

## Testing strategy

The current test file in [tests/cryptocats.test.ts](tests/cryptocats.test.ts) serves as a scaffold. It verifies that the workspace can load the Anchor program and exposes a starting point for integration testing.

A typical test case can look like this:

```ts
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

it("loads the program", async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Cryptocats as Program;
  expect(program.programId.toBase58()).toBeDefined();
});
```

A complete test suite should cover:

- Double-claim prevention.
- Insufficient balance rejection.
- Event expiry and event start checks.
- Max-claims enforcement.
- Concurrent claim attempts from multiple wallets.
- Trait derivation stability and expected ranges.

## Known limitations of this scaffold

This repository is a solid foundation, but it is not yet a turnkey production deployment for mainnet. The main limitations are:

- The current randomness is deterministic and not verifiably unpredictable.
- The NFT metadata pipeline is not yet fully integrated with Metaplex.
- The token eligibility logic is intentionally simplified.
- The frontend is a minimal shell rather than a full wallet experience.
- The renderer is off-chain and does not yet produce metadata JSON directly from on-chain trait state.

## Recommended roadmap

Phase 1: Foundation
- Finish the Anchor program wiring and add real integration tests.
- Add a proper config file for claim policy and event defaults.
- Add safer PDA and account validation.

```rust
// Example future extension: enforce admin-only event creation
require!(ctx.accounts.authority.key() == ctx.accounts.config.authority, ErrorCode::Unauthorized);
```

Phase 2: Production readiness
- Replace the deterministic trait source with Switchboard VRF.
- Integrate Metaplex Token Metadata or Core for full NFT metadata support.
- Add indexer or snapshot-based token ownership tracking.

```bash
# Example future metadata pipeline step
python3 scripts/generate_metadata.py --traits 0 1 2 3 4
```

Phase 3: Product layer
- Build a wallet-connected frontend with claim UX and NFT gallery views.
- Add rarity dashboards, trait history, and collection statistics.
- Introduce a claim schedule and event management dashboard.

### Suggested milestone sequence

1. Milestone A — local prototype
   - Build and test the claim flow locally.
   - Validate that trait generation and SVG rendering are deterministic.

2. Milestone B — devnet deployment
   - Deploy the Anchor program to devnet.
   - Create a test event and validate wallet claiming.

3. Milestone C — real-world launch preparation
   - Integrate real randomness and metadata storage.
   - Add indexing and analytics.
   - Prepare the authority model and operational playbook.

## Repository layout

- [programs/cryptocats](programs/cryptocats) — Anchor program source.
- [programs/cryptocats/src/lib.rs](programs/cryptocats/src/lib.rs) — Core instruction handlers and account definitions.
- [app](app) — Minimal Next.js frontend.
- [assets](assets) — Trait renderer and asset tooling.
- [traits/traits.json](traits/traits.json) — Trait catalog and weights.
- [scripts](scripts) — Deployment and operational helpers.
- [tests](tests) — Test scaffolding.

## Summary

CryptoCats is a practical blueprint for a hybrid token-and-NFT experience on Solana. It combines an Anchor-based claim system, compact on-chain trait state, and a simple renderer to create a foundation for a collectible NFT product around a community token. The repository is intentionally structured to support the next steps of production hardening: verifiable randomness, proper metadata integration, stronger anti-abuse measures, and a polished frontend experience.
