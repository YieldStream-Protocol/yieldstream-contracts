# Vault Mathematics Specification

YieldStream uses a standard fractional-reserve model to account for user yield.

Instead of updating individual user balances every time yield is harvested, the Vault simply holds the yield. Because the total number of `ysTokens` (Total Shares) remains constant while the Vault's underlying balance increases, the value of each individual `ysToken` intrinsically goes up.

## Deposit Calculation
When a user deposits the underlying asset, the contract mints new `ysTokens` based on the current exchange rate:

`Shares to Mint = (Deposit Amount * Total Shares) / Total Vault Balance`

*Note: On the very first deposit, when Total Shares and Vault Balance are 0, the exchange rate defaults to 1:1.*

## Withdrawal Calculation
When a user burns their `ysTokens`, they receive their proportional slice of the total underlying assets (which includes their principal + their share of the accrued yield):

`Underlying to Return = (Shares to Burn * Total Vault Balance) / Total Shares`