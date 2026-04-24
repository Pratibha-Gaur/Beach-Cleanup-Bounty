# Beach Cleanup Bounty

A Soroban smart contract on the Stellar network that rewards volunteers based on the amount of beach waste they collect.

---

## Project Description

**Beach Cleanup Bounty** is a decentralized environmental incentive platform built using **Soroban Smart Contracts** on the **Stellar blockchain**.

The goal of the project is to encourage volunteers to clean beaches by rewarding them according to the **kilograms of waste collected**.

Sponsors initialize a cleanup campaign by defining:

* **Reward per kilogram**
* **Total campaign funds**

When a volunteer submits the amount of waste collected, the smart contract automatically calculates the reward and deducts it from the campaign balance.

This creates a transparent, fair, and tamper-proof reward mechanism for environmental cleanup initiatives.

---

## What it does

The smart contract provides the following functionalities:

### 1. Initialize a Campaign

A sponsor creates a campaign by setting:

* `reward_per_kg` → amount rewarded per kilogram of waste
* `total_funds` → total reward pool available

Example:

If:

* reward per kg = `2`
* total funds = `1000`

Then volunteers earn **2 tokens per kilogram**, up to a total pool of **1000 tokens**.

---

### 2. Submit Cleanup Work

A volunteer submits the amount of waste collected in kilograms.

The contract calculates:

`reward = kilograms_collected × reward_per_kg`

Example:

If a volunteer collects `10 kg`:
`10 × 2 = 20 tokens`

The volunteer earns **20 tokens**, and the campaign pool decreases accordingly.

---

### 3. Track Volunteer Rewards

The contract stores each volunteer’s earned rewards on-chain, making the process transparent and auditable.

---

### 4. Monitor Campaign Funds

Anyone can check:

* reward rate
* remaining campaign funds

This ensures transparency for sponsors and participants.

---

## Features

### **Automated Reward Calculation**

Rewards are calculated automatically based on the quantity of waste collected.

### **Transparent Fund Management**

Campaign funds are reduced transparently after each cleanup submission.

### **Volunteer Reward Tracking**

Each volunteer’s earned reward is stored on-chain.

### **Decentralized Accountability**

All campaign data is maintained on the blockchain for trust and transparency.

### **Environmental Impact Incentive**

Creates a financial incentive for volunteers to help keep beaches clean.

---

## Smart Contract Functions

### `init_campaign(reward_per_kg, total_funds)`

Initializes a cleanup campaign.

---

### `submit_cleanup(volunteer, kilograms)`

Calculates and records reward for the volunteer.

---

### `get_reward(volunteer)`

Returns the volunteer’s earned reward.

---

### `get_campaign()`

Returns campaign information:

* reward per kg
* remaining funds

---

## Example Workflow

### Step 1:

Initialize campaign:

`init_campaign(2, 1000)`

Campaign reward:

* **2 tokens/kg**
* **1000 total tokens**

---

### Step 2:

Volunteer submits:
`submit_cleanup(volunteer_address, 10)`

Reward:
`10 × 2 = 20 tokens`

---

### Step 3:
Remaining campaign balance:

`980 tokens`

---

## Tech Stack

* **Rust**
* **Soroban SDK**
* **Stellar Blockchain**
* **Stellar IDE**

---

## Future Improvements

* Real Stellar token payouts using token transfer contracts
* Proof of cleanup verification via photo upload
* GPS-based location verification
* Reputation score for volunteers
* NFT badges for contributors
* Sponsor dashboard for campaign analytics

---

## Vision

To build a **blockchain-powered environmental reward system** that motivates communities to clean beaches while ensuring fair and transparent incentives for volunteers.

By combining **sustainability + blockchain**, Beach Cleanup Bounty aims to create measurable environmental impact through decentralized rewards.

---

Wallet address: GCBOGZNTQ737WRT5K6O7NBPAKPRAS2IJ2QODBAR3F2KZ3FPVEQWQVL3Y

Contract address: CBYPCK6DKSRSR4PC5G5JFUE64AW7MSNMTO55KXTVNLNKNVZCDAADMFLS

https://stellar.expert/explorer/testnet/contract/CBYPCK6DKSRSR4PC5G5JFUE64AW7MSNMTO55KXTVNLNKNVZCDAADMFLS

<img width="1900" height="971" alt="image" src="https://github.com/user-attachments/assets/d2f0c0cf-a8c6-46a6-97ac-1b7555067abb" />

