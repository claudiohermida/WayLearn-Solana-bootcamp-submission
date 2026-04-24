# Bowling Shoe Deposit (Solana Anchor Program)

This program manages a virtual deposit of bowling shoes on the Solana blockchain. It allows an owner to initialize a deposit and manage inventory (adding, borrowing, returning, and eliminating shoes) based on gender and size.

*(Note: While Anchor allows you to create separate PDA accounts for every individual item, this program is designed to use a single PDA `BowlingShoeDeposit` account that manages an internal `Vec<Shoe>` struct, ensuring all shoe records stay closely grouped under one deposit.)*

### Note: 
This porgram has been tested on Solana playground and deployed on devnet.

---


## Enums and Structures

### `Gender` (Enum)
Represents the gender classification of the shoe.
- `Male`
- `Female`

### `Size` (Enum)
Represents the size of the shoe.
- `Six`, `Seven`, `Eight`, `Nine`, `Ten`, `Eleven`

### `Shoe` (Struct)
The internal structure saved inside the deposit. 
- `gender: Gender`: The gender category.
- `size: Size`: The size of the shoe.
- `quantity: u32`: The absolute total of these shoes owned by the deposit.
- `available: u32`: The number of shoes currently not being borrowed.

### `BowlingShoeDeposit` (Account)
The primary PDA generated for the user who creates it.
- `owner: Pubkey`: The wallet address that owns and controls the deposit.
- `shoes: Vec<Shoe>`: An array containing the inventory of shoes (Max combinations: 15).

---

## Instructions (Functions)

### `create_deposit`
Initializes the `BowlingShoeDeposit` PDA for the signing wallet. It sets the caller as the `owner` and prepares an empty vector to begin storing `Shoe` structs.

### `add_shoe(gender, size)`
Looks for the specific `(gender, size)` combination in the deposit. 
- If found, it increments both the `quantity` and `available` counts by 1.
- If not found, it pushes a new `Shoe` into the vector with `quantity: 1` and `available: 1`.

### `fetch_available_shoes(gender, size)`
Iterates through the vector to locate the requested shoe type and returns its `available` count. If the shoe type has never been added, it defaults to returning `0`.

### `borrow_shoe(gender, size)`
Allows a shoe to be "checked out". 
- Finds the shoe and verifies that `available > 0`.
- If true, it decreases `available` by 1. The `quantity` (total owned) remains unaffected.
- Throws `Error::NoShoeAvailable` if none are left, or `Error::NotExistentShoe` if the shoe isn't in the inventory.

### `return_shoe(gender, size)`
Allows a checked-out shoe to be returned to the deposit.
- Finds the shoe and ensures `available < quantity` (meaning at least one is currently borrowed).
- If true, it increases `available` by 1.
- Throws `Error::ShoeNotBorrowed` if all shoes of this type are already in the deposit.

### `eliminate_shoe(gender, size)`
Permanently removes a shoe from the total inventory.
- Checks that both `quantity >= 1` and `available >= 1` (you cannot eliminate a shoe that is currently being borrowed).
- Decreases both `quantity` and `available` by 1.
- Throws `Error::NotExistentShoe` if the conditions aren't met or the shoe doesn't exist.

---

## Error Codes
- `NoEresElOwner`: Thrown when a user attempts to mutate a deposit they did not create.
- `NoShoeAvailable`: Thrown when trying to borrow a shoe that is out of stock.
- `ShoeNotBorrowed`: Thrown when trying to return a shoe, but the deposit already has its maximum quantity.
- `NotExistentShoe`: Thrown when trying to borrow, return, or eliminate a shoe type that has not been registered or has 0 stock.