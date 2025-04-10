# Escrow

1) Maker creates escrow
  - Asks certain amount of TOKEN_A for certain amount of TOKEN_B
2) Taker deposits the certain amount of TOKEN_B
3) After deposit
  - Maker gets the TOKEN_B
  - Taker gets the TOKEN_A
  - Close vault
4) Maker can do a refund, if it doesn't like the escrow anymore
   - Funds get transfered back to maker
   - Close vault


In the escrow `seed` makes it able to the user to make different escrow for the same ATA

Make
- init escrow
- deposit funds

