# CENNZ_ED_Kickstarter

## CENNZ_ED Event Kickstarter Project

### OVERVIEW OF CENNZ KICKSTARTER PROJECT:

##### SINGLE CONTROLLING CONTRACT
1. CREATOR_ACCOUNT_ADDRESS (Does not change)
2. Create unique PROJECT_ID (new ID incremented +1 for each new project)
3. PERCENTAGE of transfer to return to CREATOR_ACCOUNT_ADDRESS on payout to PROJECT_ACCOUNT_ADDRESS (profits)
4. DELAY for payouts

Note: The contract needs to specifically emit details so it can be called

##### PROJECT
1. PROJECT_ID
2. Project owner PROJECT_ACCOUNT_ADDRESS
3. MIN_CRYPTO contribution
4. TOTAL_CRYPTO sum of contributions
5. PAYOUT_CALL (BINARY - YES/NO) if TOTAL_CRYPTO > MIN_CRYPTO then PROJECT_ACCOUNT_ADDRESS can make PAYOUT_CALL can = YES, otherwise NO
6. START_TIME
7. END_TIME if reached, wait DELAY, then if PAYOUT_CALL is YES, transfer TOTAL_CRYPTO less PERCENTAGE to PROJECT_ACCOUNT_ADDRESS, if PAYOUT_CALL is NO, transfer MY_CRYPTO to FUNDER_ID (crypto is returned to the participant)

##### PARTICIPANT
1. PROJECT_ID for project
2. FUNDER_ID Participant identified by wallet
3. MY_CRYPTO participant contribution amount

##### Need a front end that:
1. Initiates a project
2. That makes calls to the contract and reports contract details


**In preparation to deploy: in ./src directory:**

```
cargo generate \
    --name KickStarter \
    --git https://github.com/cennznet/contract-template

# Use rust nightly toolchain for contract dev.
cd KickStarter && rustup override set nightly
```
