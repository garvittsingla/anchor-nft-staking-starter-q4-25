Nft Staking
- anchor-nft-staking-starter-q4-25/programs/anchor-nft-staking-q4-25/src/instructions/stake.rs:30: collection constraint checks asset.owner instead of collection.owner, so collection ownership is not validated.
- anchor-nft-staking-starter-q4-25/programs/anchor-nft-staking-q4-25/src/instructions/stake.rs:31: collection constraint checks asset.data_is_empty() instead of collection.data_is_empty(), so collection initialization is not validated.
- anchor-nft-staking-starter-q4-25/programs/anchor-nft-staking-q4-25/src/instructions/unstake.rs:30: collection constraint checks asset.data_is_empty() instead of collection.data_is_empty(), so collection initialization is not validated.
- anchor-nft-staking-starter-q4-25/programs/anchor-nft-staking-q4-25/src/instructions/unstake.rs:73: points are computed with time_elapsed + points_per_stake; expected to multiply days staked by points_per_stake (example: let points_earned = time_elapsed * self.config.points_per_stake as u32;).
- anchor-nft-staking-starter-q4-25/programs/anchor-nft-staking-q4-25/src/instructions/unstake.rs:57: core_program lacks an address constraint for CORE_PROGRAM_ID.
work has to be done
